// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Block, Error, ExprClosure, Pat, Result, Stmt, Token, parse_macro_input};

struct Lambda {
    captures: Block,
    closure: ExprClosure,
}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> Result<Self> {
        let captures = input.parse()?;
        input.parse::<Token![,]>()?;
        let closure = input.parse()?;
        input.parse::<Option<Token![,]>>()?;
        Ok(Lambda { captures, closure })
    }
}

fn expand_lambda(lambda: Lambda, is_unsafe: bool) -> Result<proc_macro2::TokenStream> {
    let mut names = Vec::new();
    let mut types = Vec::new();
    let mut inits = Vec::new();
    for stmt in &lambda.captures.stmts {
        let Stmt::Local(local) = stmt else {
            return Err(Error::new_spanned(stmt, "expected a `let` for a capture"));
        };
        let (Pat::Type(pat), Some(init)) = (&local.pat, &local.init) else {
            return Err(Error::new_spanned(
                local,
                "expected `let <name>: <type> = <init>;` for a capture",
            ));
        };
        names.push(&pat.pat);
        types.push(&pat.ty);
        inits.push(&init.expr);
    }

    let mut params = Vec::new();
    let mut param_types = Vec::new();
    for input in &lambda.closure.inputs {
        let Pat::Type(pat) = input else {
            return Err(Error::new_spanned(input, "expected a typed parameter"));
        };
        params.push(pat);
        param_types.push(&pat.ty);
    }

    let output = &lambda.closure.output;
    let body = &lambda.closure.body;
    let (receiver, body, constructor) = if is_unsafe {
        (
            quote! { &mut self },
            quote! { unsafe { #body } },
            quote! { from_lambda_unsafe },
        )
    } else {
        (quote! { &self }, quote! { #body }, quote! { from_lambda })
    };

    Ok(quote! {{
        struct __Lambda {
            #(#names: #types,)*
        }
        impl __Lambda {
            #[allow(unused_unsafe, clippy::too_many_arguments)]
            fn call(#receiver #(, #params)*) #output {
                #body
            }
        }
        ::libcc2rs::FnPtr::<fn(#(#param_types),*) #output>::#constructor(
            __Lambda { #(#names: #inits,)* },
            __Lambda::call,
        )
    }})
}

pub fn expand(input: TokenStream, is_unsafe: bool) -> TokenStream {
    let lambda = parse_macro_input!(input as Lambda);
    expand_lambda(lambda, is_unsafe)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
