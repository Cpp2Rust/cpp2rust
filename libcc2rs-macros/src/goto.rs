// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Lifetime, Token};

use crate::state_machine::{Arm, GotoStateMachine, StateMachine};

pub fn expand(input: TokenStream) -> TokenStream {
    let GotoBlockInput { arms } = parse_macro_input!(input as GotoBlockInput);
    GotoStateMachine {
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
        let mut arms = Vec::new();
        while !input.is_empty() {
            let label: Lifetime = input.parse()?;
            input.parse::<Token![=>]>()?;
            let body: Expr = input.parse()?;
            arms.push(GotoArm { label, body });
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { arms })
    }
}
