// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::visit_mut::{self, VisitMut};
use syn::{Expr, ExprBreak, ExprContinue, Lifetime, Pat};

pub struct Arm {
    pub label: String,
    pub body: Expr,
}

pub struct DispatchCase {
    pub pat: Pat,
    pub guard: Option<Expr>,
    pub target: String,
}

pub trait StateMachine {
    fn emit(self) -> TokenStream2;
}

fn sm_label() -> Lifetime {
    Lifetime::new("'__sm", Span::call_site())
}

// Collection of labeled arms that fall-through by default
pub struct GotoStateMachine {
    pub arms: Vec<Arm>,
}

impl GotoStateMachine {
    // Rewrites unlabeled break / continue to { flag = true; break '__sm; }
    fn propagate_rewrite(
        body: &mut Expr,
        label: &Lifetime,
        break_flag: &Ident,
        cont_flag: &Ident,
    ) -> (bool, bool) {
        let mut br = PropagateRewriter::for_break(label.clone(), break_flag.clone());
        br.visit_expr_mut(body);
        let mut cr = PropagateRewriter::for_continue(label.clone(), cont_flag.clone());
        cr.visit_expr_mut(body);
        (br.found, cr.found)
    }

    // idx => { body; tail }
    fn emit_body_arm(
        idx: u32,
        body: &Expr,
        is_last: bool,
        label: &Lifetime,
        state: &Ident,
    ) -> TokenStream2 {
        let tail = if is_last {
            quote! { break #label; }
        } else {
            let next = idx + 1;
            quote! { #state = #next; continue #label; }
        };
        quote! {
            #idx => {
                #[allow(unreachable_code)]
                { #body; #tail }
            }
        }
    }

    fn bailout(any: bool, flag: &Ident, stmt: TokenStream2) -> (TokenStream2, TokenStream2) {
        if !any {
            return (TokenStream2::new(), TokenStream2::new());
        }
        (
            quote! { let mut #flag: bool = false; },
            quote! {
                #[allow(unreachable_code)]
                if #flag { #stmt }
            },
        )
    }
}

impl StateMachine for GotoStateMachine {
    fn emit(self) -> TokenStream2 {
        let lbl = sm_label();
        let s = format_ident!("__s");
        let break_flag = format_ident!("__user_break");
        let cont_flag = format_ident!("__user_continue");

        let n = self.arms.len();
        let mut any_break = false;
        let mut any_continue = false;
        let body_arms: Vec<_> = self
            .arms
            .iter()
            .enumerate()
            .map(|(i, arm)| {
                let mut body = arm.body.clone();
                let (had_br, had_cn) =
                    Self::propagate_rewrite(&mut body, &lbl, &break_flag, &cont_flag);
                any_break |= had_br;
                any_continue |= had_cn;
                Self::emit_body_arm(i as u32, &body, i + 1 == n, &lbl, &s)
            })
            .collect();

        let (brk_decl, brk_bailout) = Self::bailout(any_break, &break_flag, quote! { break; });
        let (cnt_decl, cnt_bailout) = Self::bailout(any_continue, &cont_flag, quote! { continue; });

        quote! {{
            #brk_decl
            #cnt_decl
            let mut #s: u32 = 0;
            #[allow(unreachable_code, unused_labels)]
            #lbl: loop {
                match #s {
                    #(#body_arms)*
                    _ => break #lbl,
                }
            }
            #brk_bailout
            #cnt_bailout
        }}
    }
}

// GotoStateMachine(dispatch arm + cases)
pub struct SwitchStateMachine {
    pub goto: GotoStateMachine,
    pub condition: Expr,
    pub cases: Vec<DispatchCase>,
}

impl SwitchStateMachine {
    // Rewrite break into break '__sm
    fn convert_break_to_switch_exit(arms: &Vec<Arm>, label: &Lifetime) -> Vec<Arm> {
        arms.into_iter()
            .map(|a| {
                let mut body = a.body.clone();
                ExitSwitchRewriter {
                    label: label.clone(),
                }
                .visit_expr_mut(&mut body);
                Arm {
                    label: a.label.clone(),
                    body,
                }
            })
            .collect()
    }

    fn build_dispatch_arm(&self, user_arms: &[Arm], label: &Lifetime, state: &Ident) -> Arm {
        let cond = &self.condition;
        let case_arms = self.cases.iter().map(|c| {
            let target_pos = user_arms
                .iter()
                .position(|a| a.label == c.target)
                .expect("dispatch target must reference an arm label");
            let idx = (target_pos as u32) + 1;
            let pat = &c.pat;
            let guard = c.guard.as_ref().map(|g| quote! { if #g });
            quote! { #pat #guard => { #state = #idx; continue #label; } }
        });
        let body: Expr = syn::parse_quote! {
            {
                #[allow(unreachable_patterns)]
                match #cond {
                    #(#case_arms,)*
                    _ => break #label,
                }
            }
        };
        Arm {
            label: "__dispatch".into(),
            body,
        }
    }
}

impl StateMachine for SwitchStateMachine {
    fn emit(self) -> TokenStream2 {
        let lbl = sm_label();
        let s = format_ident!("__s");

        let user_arms = Self::convert_break_to_switch_exit(&self.goto.arms, &lbl);
        let dispatch = self.build_dispatch_arm(&user_arms, &lbl, &s);

        let mut arms = Vec::new();
        arms.push(dispatch);
        arms.extend(user_arms);

        GotoStateMachine { arms }.emit()
    }
}

// Rewrite break into break '__sm
struct ExitSwitchRewriter {
    label: Lifetime,
}

impl VisitMut for ExitSwitchRewriter {
    fn visit_expr_break_mut(&mut self, node: &mut ExprBreak) {
        if node.label.is_none() {
            node.label = Some(self.label.clone());
        }
    }
    fn visit_expr_loop_mut(&mut self, _: &mut syn::ExprLoop) {}
    fn visit_expr_while_mut(&mut self, _: &mut syn::ExprWhile) {}
    fn visit_expr_for_loop_mut(&mut self, _: &mut syn::ExprForLoop) {}
}

enum ControlKind {
    Break,
    Continue,
}

// Rewrites unlabeled break / continue to { flag = true; break '__sm; }
struct PropagateRewriter {
    label: Lifetime,
    flag: Ident,
    kind: ControlKind,
    found: bool,
}

impl PropagateRewriter {
    fn for_break(label: Lifetime, flag: Ident) -> Self {
        Self {
            label,
            flag,
            kind: ControlKind::Break,
            found: false,
        }
    }
    fn for_continue(label: Lifetime, flag: Ident) -> Self {
        Self {
            label,
            flag,
            kind: ControlKind::Continue,
            found: false,
        }
    }
}

impl VisitMut for PropagateRewriter {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let hit = match self.kind {
            ControlKind::Break => {
                matches!(expr, Expr::Break(ExprBreak { label: None, .. }))
            }
            ControlKind::Continue => {
                matches!(expr, Expr::Continue(ExprContinue { label: None, .. }))
            }
        };
        if hit {
            self.found = true;
            let flag = &self.flag;
            let lbl = &self.label;
            *expr = syn::parse_quote! {
                { #flag = true; break #lbl; }
            };
            return;
        }
        visit_mut::visit_expr_mut(self, expr);
    }
    fn visit_expr_loop_mut(&mut self, _: &mut syn::ExprLoop) {}
    fn visit_expr_while_mut(&mut self, _: &mut syn::ExprWhile) {}
    fn visit_expr_for_loop_mut(&mut self, _: &mut syn::ExprForLoop) {}
}
