// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// switch!(match <condition> {
//     <pat> [if <guard>] => <body>,
//     <pat> [if <guard>] => <body>,
//     ...
//     _ => <body>,
// });
//
// The input is a real `match` expression so rustfmt formats it normally.

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Pat};

use crate::state_machine::{emit_state_machine, Arm, ArmEntry};

pub fn expand(input: TokenStream) -> TokenStream {
    let SwitchInput { condition, arms } = parse_macro_input!(input as SwitchInput);
    emit_state_machine(
        Some(condition),
        arms.into_iter()
            .enumerate()
            .map(|(i, a)| Arm {
                label: format!("__c{}", i),
                entry: ArmEntry::Dispatch {
                    pat: a.pat,
                    guard: a.guard,
                },
                body: a.body,
            })
            .collect(),
    )
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
