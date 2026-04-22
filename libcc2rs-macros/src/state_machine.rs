// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::visit_mut::{self, VisitMut};
use syn::{Expr, ExprBreak, Lifetime, Pat};

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
        let body = rewrite_body(&arm.body, &lbl);
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

fn rewrite_body(body: &Expr, label: &Lifetime) -> TokenStream2 {
    let mut body = body.clone();
    BreakRewriter {
        label: label.clone(),
    }
    .visit_expr_mut(&mut body);
    quote! { #body }
}

struct BreakRewriter {
    label: Lifetime,
}

impl VisitMut for BreakRewriter {
    fn visit_expr_break_mut(&mut self, node: &mut ExprBreak) {
        if node.label.is_none() {
            node.label = Some(self.label.clone());
        }
    }
    fn visit_expr_loop_mut(&mut self, _: &mut syn::ExprLoop) {}
    fn visit_expr_while_mut(&mut self, _: &mut syn::ExprWhile) {}
    fn visit_expr_for_loop_mut(&mut self, _: &mut syn::ExprForLoop) {}
}
