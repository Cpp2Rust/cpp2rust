// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{Block, Expr, ExprBlock, Lifetime, Stmt, parse_macro_input};

use crate::state_machine::{Arm, GotoStateMachine, StateMachineNames, StateMachine};

pub fn expand(input: TokenStream) -> TokenStream {
    let GotoBlockInput { arms } = parse_macro_input!(input as GotoBlockInput);
    GotoStateMachine {
        names: StateMachineNames::fresh(),
        arms: arms
            .into_iter()
            .map(|a| Arm {
                label: a.label.ident.to_string(),
                body: a.body,
            })
            .collect(),
    }
    .emit()
    .into()
}

struct GotoBlockInput {
    arms: Vec<GotoArm>,
}

struct GotoArm {
    label: Lifetime,
    body: Expr,
}

impl Parse for GotoBlockInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let block: Block = input.parse()?;
        let mut arms = Vec::new();
        for stmt in block.stmts {
            let Stmt::Expr(Expr::Block(eb), _) = stmt else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "goto_block! body must be a sequence of labeled blocks",
                ));
            };
            let Some(label) = eb.label else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "goto_block! arm must be a labeled block",
                ));
            };
            arms.push(GotoArm {
                label: label.name,
                body: Expr::Block(ExprBlock {
                    attrs: eb.attrs,
                    label: None,
                    block: eb.block,
                }),
            });
        }
        Ok(Self { arms })
    }
}
