// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Punct, Spacing, TokenTree};
use quote::quote;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    Block, Error, ExprClosure, FieldValue, Pat, Result, Stmt, Token, braced, parse_macro_input,
};

type CaptureInits = Punctuated<FieldValue, Token![,]>;

struct Lambda {
    captures: Block,
    closure: ExprClosure,
    copy_from: Option<CaptureInits>,
    move_from: Option<CaptureInits>,
}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> Result<Self> {
        let captures = input.parse()?;
        input.parse::<Token![,]>()?;
        let closure = input.parse()?;
        input.parse::<Option<Token![,]>>()?;
        let mut copy_from = None;
        let mut move_from = None;
        while !input.is_empty() {
            let name: Ident = input.parse()?;
            let inits;
            braced!(inits in input);
            let inits = inits.parse_terminated(FieldValue::parse, Token![,])?;
            if name == "copy_from" {
                copy_from = Some(inits);
            } else if name == "move_from" {
                move_from = Some(inits);
            } else {
                return Err(Error::new(
                    name.span(),
                    "expected `copy_from` or `move_from`",
                ));
            }
            input.parse::<Option<Token![,]>>()?;
        }
        Ok(Lambda {
            captures,
            closure,
            copy_from,
            move_from,
        })
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

fn expand_constructor(
    inits: &Option<CaptureInits>,
    names: &HashSet<String>,
    is_unsafe: bool,
) -> Result<proc_macro2::TokenStream> {
    let Some(inits) = inits else {
        return Ok(quote! { unreachable!("the lambda has no such constructor") });
    };
    let mut fields = Vec::new();
    for init in inits {
        let member = &init.member;
        let expr = &init.expr;
        let expr = rewrite_captures(quote! { #expr }, names, false)?;
        fields.push(quote! { #member: #expr });
    }
    let body = quote! { __Lambda { #(#fields,)* } };
    Ok(if is_unsafe {
        quote! { unsafe { #body } }
    } else {
        body
    })
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
    let copy_from = expand_constructor(&lambda.copy_from, &captured, is_unsafe)?;
    let move_from = expand_constructor(&lambda.move_from, &captured, is_unsafe)?;
    let (receiver, body, constructor) = if is_unsafe {
        (
            quote! { &mut self },
            quote! { unsafe { #body } },
            quote! { from_lambda_unsafe },
        )
    } else {
        (quote! { &self }, body, quote! { from_lambda })
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
            #[allow(unused_unsafe)]
            fn copy_from(#receiver) -> Self {
                #copy_from
            }
            #[allow(unused_unsafe)]
            fn move_from(#receiver) -> Self {
                #move_from
            }
        }
        ::libcc2rs::FnPtr::<fn(#(#param_types),*) #output>::#constructor(
            __Lambda { #(#names: #inits,)* },
            __Lambda::call,
            __Lambda::copy_from,
            __Lambda::move_from,
        )
    }})
}

pub fn expand(input: TokenStream, is_unsafe: bool) -> TokenStream {
    let lambda = parse_macro_input!(input as Lambda);
    expand_lambda(lambda, is_unsafe)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
