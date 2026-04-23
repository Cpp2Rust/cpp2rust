// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::visit_mut::{self, VisitMut};
use syn::{Expr, Lifetime, Pat};

pub struct Arm {
    pub label: String,
    pub entry: ArmEntry,
    pub body: Expr,
}

pub enum ArmEntry {
    Dispatch { pat: Pat, guard: Option<Expr> },
    LabelOnly,
}

pub fn emit_state_machine(condition: Option<Expr>, arms: Vec<Arm>) -> TokenStream2 {
    let lbl = Lifetime::new("'__sm", proc_macro2::Span::call_site());
    let s = format_ident!("__s");

    let base = if condition.is_some() { 1u32 } else { 0u32 };
    let rewrite_switch_break = condition.is_some();

    let dispatch_arm = condition.map(|scrut| {
        let case_arms = arms
            .iter()
            .enumerate()
            .filter_map(|(i, arm)| match &arm.entry {
                ArmEntry::Dispatch { pat, guard } => {
                    let idx = base + i as u32;
                    let guard = guard.as_ref().map(|g| quote! { if #g });
                    Some(quote! { #pat #guard => { #s = #idx; continue #lbl; } })
                }
                ArmEntry::LabelOnly => None,
            });
        quote! {
            0u32 => {
                #[allow(unreachable_patterns)]
                match #scrut {
                    #(#case_arms,)*
                    _ => break #lbl,
                }
            }
        }
    });

    let n = arms.len();
    let body_arms = arms.iter().enumerate().map(|(i, arm)| {
        let idx = base + i as u32;
        let body = rewrite_body(&arm.body, &lbl, rewrite_switch_break);
        let tail = if i + 1 < n {
            let next = idx + 1;
            quote! { #s = #next; continue #lbl; }
        } else {
            quote! { break #lbl; }
        };
        quote! {
            #idx => {
                #[allow(unreachable_code)]
                { #body; #tail }
            }
        }
    });

    quote! {{
        let mut #s: u32 = 0;
        #[allow(unreachable_code, unused_labels)]
        #lbl: loop {
            match #s {
                #dispatch_arm
                #(#body_arms)*
                _ => break #lbl,
            }
        }
    }}
}

fn rewrite_body(body: &Expr, label: &Lifetime, rewrite_switch_break: bool) -> TokenStream2 {
    let mut body = body.clone();
    LoopControlForbidden.visit_expr_mut(&mut body);
    if rewrite_switch_break {
        SwitchBreakRewriter {
            label: label.clone(),
        }
        .visit_expr_mut(&mut body);
    }
    quote! { #body }
}

// Rewrites top-level switch_break!() into break '__sm. switch_break!() inside a loop is a compile
// error.
struct SwitchBreakRewriter {
    label: Lifetime,
}

impl SwitchBreakRewriter {
    fn replacement(&self) -> Expr {
        let lbl = &self.label;
        syn::parse_quote! { break #lbl }
    }
}

impl VisitMut for SwitchBreakRewriter {
    fn visit_stmt_mut(&mut self, stmt: &mut syn::Stmt) {
        if let syn::Stmt::Macro(sm) = stmt {
            if sm.mac.path.is_ident("switch_break") {
                *stmt = syn::Stmt::Expr(self.replacement(), sm.semi_token);
                return;
            }
        }
        visit_mut::visit_stmt_mut(self, stmt);
    }
    fn visit_expr_loop_mut(&mut self, _: &mut syn::ExprLoop) {}
    fn visit_expr_while_mut(&mut self, _: &mut syn::ExprWhile) {}
    fn visit_expr_for_loop_mut(&mut self, _: &mut syn::ExprForLoop) {}
}

// Forbid user-written break/continue. We want to hide the fact that switch! and goto_block!
// are loops behind the scenes.
struct LoopControlForbidden;

impl VisitMut for LoopControlForbidden {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Break(_) => {
                *expr = syn::parse_quote! {
                    compile_error!(
                        "break is not allowed at the top level of a switch!/goto_block! arm. Inside a switch! use switch_break!()",
                    )
                };
                return;
            }
            Expr::Continue(_) => {
                *expr = syn::parse_quote! {
                    compile_error!(
                        "continue is not allowed at the top level of a switch!/goto_block! arm"
                    )
                };
                return;
            }
            _ => {}
        }
        visit_mut::visit_expr_mut(self, expr);
    }
    fn visit_expr_loop_mut(&mut self, _: &mut syn::ExprLoop) {}
    fn visit_expr_while_mut(&mut self, _: &mut syn::ExprWhile) {}
    fn visit_expr_for_loop_mut(&mut self, _: &mut syn::ExprForLoop) {}
}
