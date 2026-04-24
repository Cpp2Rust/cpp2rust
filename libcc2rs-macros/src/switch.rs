// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Pat};

use crate::state_machine::{Arm, DispatchCase, GotoStateMachine, StateMachine, SwitchStateMachine};

pub fn expand(input: TokenStream) -> TokenStream {
    let SwitchInput { condition, arms } = parse_macro_input!(input as SwitchInput);
    let mut cases = Vec::with_capacity(arms.len());
    let mut cfg_arms = Vec::with_capacity(arms.len());
    for (i, a) in arms.into_iter().enumerate() {
        let label = format!("__c{}", i);
        cases.push(DispatchCase {
            pat: a.pat,
            guard: a.guard,
            target: label.clone(),
        });
        cfg_arms.push(Arm {
            label,
            body: a.body,
        });
    }
    SwitchStateMachine {
        goto: GotoStateMachine { arms: cfg_arms },
        condition: condition,
        cases,
    }
    .emit()
    .into()
}

struct SwitchInput {
    condition: Expr,
    arms: Vec<SwitchArm>,
}

struct SwitchArm {
    pat: Pat,
    guard: Option<Expr>,
    body: Expr,
}

impl Parse for SwitchInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let m: syn::ExprMatch = input.parse()?;
        Ok(Self {
            condition: *m.expr,
            arms: m
                .arms
                .into_iter()
                .map(|a| SwitchArm {
                    pat: a.pat,
                    guard: a.guard.map(|(_if, e)| *e),
                    body: *a.body,
                })
                .collect(),
        })
    }
}
