// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Punct, Spacing, TokenTree};
use quote::quote;
use std::collections::HashSet;
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

fn is_punct(token: Option<&TokenTree>, ch: char) -> bool {
    matches!(token, Some(TokenTree::Punct(punct)) if punct.as_char() == ch)
}

fn is_ident(token: Option<&TokenTree>, name: &str) -> bool {
    matches!(token, Some(TokenTree::Ident(ident)) if ident == name)
}

fn is_lambda_macro(name: Option<&TokenTree>, bang: Option<&TokenTree>) -> bool {
    is_punct(bang, '!') && (is_ident(name, "lambda") || is_ident(name, "lambda_unsafe"))
}

fn rewrite_nested_lambda(
    tokens: proc_macro2::TokenStream,
    names: &HashSet<String>,
) -> Result<proc_macro2::TokenStream> {
    let mut tokens = tokens.into_iter();
    let mut out = Vec::new();
    if let Some(TokenTree::Group(captures)) = tokens.next() {
        let mut group = Group::new(
            captures.delimiter(),
            rewrite_captures(captures.stream(), names, true)?,
        );
        group.set_span(captures.span());
        out.push(TokenTree::Group(group));
    }
    out.extend(tokens);
    Ok(out.into_iter().collect())
}

fn rewrite_captures(
    tokens: proc_macro2::TokenStream,
    names: &HashSet<String>,
    in_capture_block: bool,
) -> Result<proc_macro2::TokenStream> {
    let tokens: Vec<TokenTree> = tokens.into_iter().collect();
    let mut out = Vec::new();
    let mut in_params = false;
    for (i, token) in tokens.iter().enumerate() {
        let prev = i.checked_sub(1).map(|j| &tokens[j]);
        let prev2 = i.checked_sub(2).map(|j| &tokens[j]);
        let next = tokens.get(i + 1);
        let next2 = tokens.get(i + 2);
        match token {
            TokenTree::Group(group) => {
                let stream = if is_lambda_macro(prev2, prev) {
                    rewrite_nested_lambda(group.stream(), names)?
                } else {
                    rewrite_captures(group.stream(), names, in_capture_block)?
                };
                let mut rewritten = Group::new(group.delimiter(), stream);
                rewritten.set_span(group.span());
                out.push(TokenTree::Group(rewritten));
            }
            TokenTree::Punct(punct) if punct.as_char() == '|' => {
                if in_params {
                    in_params = false;
                } else if is_punct(next, '|')
                    || (matches!(next, Some(TokenTree::Ident(_))) && is_punct(next2, ':'))
                {
                    in_params = true;
                }
                out.push(token.clone());
            }
            TokenTree::Ident(ident) if names.contains(&ident.to_string()) => {
                let before_colon = is_punct(next, ':') && !is_punct(next2, ':');
                let is_let =
                    is_ident(prev, "let") || (is_ident(prev, "mut") && is_ident(prev2, "let"));
                if !in_capture_block && (is_let || (in_params && before_colon)) {
                    return Err(Error::new(
                        ident.span(),
                        format!("`{ident}` shadows a capture of the lambda"),
                    ));
                }
                let is_member = is_punct(prev, '.') && !is_punct(prev2, '.');
                let is_path = is_punct(prev, ':') && is_punct(prev2, ':');
                let is_label = is_punct(prev, '\'');
                let is_macro = is_punct(next, '!') && !is_punct(next2, '=');
                if is_member || is_path || is_label || is_macro || before_colon {
                    out.push(token.clone());
                } else {
                    out.push(TokenTree::Ident(Ident::new("self", ident.span())));
                    out.push(TokenTree::Punct(Punct::new('.', Spacing::Alone)));
                    out.push(token.clone());
                }
            }
            _ => out.push(token.clone()),
        }
    }
    Ok(out.into_iter().collect())
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
        let Pat::Ident(name) = &*pat.pat else {
            return Err(Error::new_spanned(pat, "expected a name for a capture"));
        };
        names.push(&name.ident);
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
    let captured = names.iter().map(|name| name.to_string()).collect();
    let body = rewrite_captures(quote! { #body }, &captured, false)?;
    // The copy and move constructors of the lambda copy and move each
    // capture, and its destructor destroys them in reverse order, as chosen
    // from their types by the traits in libcc2rs::__capture.
    let reversed: Vec<_> = names.iter().rev().collect();
    let (receiver, body, destroy, constructor, copy_from, move_from) = if is_unsafe {
        (
            quote! { &mut self },
            quote! { unsafe { #body } },
            quote! {
                use ::libcc2rs::__capture::{DestroyNoneUnsafe as _, DestroyWithDtorUnsafe as _};
                unsafe {
                    #((&&::libcc2rs::__capture::CaptureUnsafe(
                        &raw mut self.#reversed
                    )).destroy_capture();)*
                }
            },
            quote! { from_lambda_unsafe },
            quote! {
                use ::libcc2rs::__capture::{
                    CopyCloneUnsafe as _, CopyLambdaUnsafe as _, CopyNoneUnsafe as _,
                };
                unsafe {
                    __Lambda { #(#names: (&&&::libcc2rs::__capture::CaptureUnsafe(
                        &raw mut self.#names
                    )).copy_capture(),)* }
                }
            },
            quote! {
                use ::libcc2rs::__capture::{
                    MoveCloneUnsafe as _, MoveNoneUnsafe as _, MoveWithCtorUnsafe as _,
                };
                unsafe {
                    __Lambda { #(#names: (&&&::libcc2rs::__capture::CaptureUnsafe(
                        &raw mut self.#names
                    )).move_capture(),)* }
                }
            },
        )
    } else {
        (
            quote! { &self },
            body,
            quote! {
                use ::libcc2rs::__capture::{
                    DestroyElems as _, DestroyNone as _, DestroyWithDtor as _,
                };
                #((&&&::libcc2rs::__capture::Capture(&self.#reversed)).destroy_capture();)*
            },
            quote! { from_lambda },
            quote! {
                use ::libcc2rs::__capture::{CopyClone as _, CopyLambda as _, CopyNone as _};
                __Lambda { #(#names: (&&&::libcc2rs::__capture::Capture(
                    &self.#names
                )).copy_capture(),)* }
            },
            quote! {
                use ::libcc2rs::__capture::{
                    MoveClone as _, MoveElems as _, MoveNone as _, MoveWithCtor as _,
                };
                __Lambda { #(#names: (&&&&::libcc2rs::__capture::Capture(
                    &self.#names
                )).move_capture(),)* }
            },
        )
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
            #[allow(unused_imports, unused_unsafe)]
            fn copy_from(#receiver) -> Self {
                #copy_from
            }
            #[allow(unused_imports, unused_unsafe)]
            fn move_from(#receiver) -> Self {
                #move_from
            }
            #[allow(unused_imports, unused_unsafe)]
            fn destroy(#receiver) {
                #destroy
            }
        }
        ::libcc2rs::FnPtr::<fn(#(#param_types),*) #output>::#constructor(
            __Lambda { #(#names: #inits,)* },
            __Lambda::call,
            __Lambda::copy_from,
            __Lambda::move_from,
            __Lambda::destroy,
        )
    }})
}

pub fn expand(input: TokenStream, is_unsafe: bool) -> TokenStream {
    let lambda = parse_macro_input!(input as Lambda);
    expand_lambda(lambda, is_unsafe)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
