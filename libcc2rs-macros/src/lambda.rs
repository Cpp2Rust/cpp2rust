// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Punct, Spacing, TokenTree};
use quote::quote;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::{Block, Error, ExprClosure, Pat, Result, Stmt, Token, parse_macro_input};

struct Lambda {
    is_generic: bool,
    captures: Block,
    closures: Vec<ExprClosure>,
}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> Result<Self> {
        let is_generic = input.peek(syn::Ident);
        if is_generic {
            let marker: Ident = input.parse()?;
            if marker != "Generic" {
                return Err(Error::new(marker.span(), "expected `Generic`"));
            }
            input.parse::<Token![,]>()?;
        }
        let captures = input.parse()?;
        input.parse::<Token![,]>()?;
        let mut closures = vec![input.parse()?];
        while input.parse::<Option<Token![,]>>()?.is_some() && !input.is_empty() {
            closures.push(input.parse()?);
        }
        if !is_generic && closures.len() != 1 {
            return Err(Error::new_spanned(
                &closures[1],
                "a non-generic lambda has one closure",
            ));
        }
        Ok(Lambda {
            is_generic,
            captures,
            closures,
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

    let captured: HashSet<String> = names.iter().map(|name| name.to_string()).collect();
    let mut calls = Vec::new();
    let mut sigs = Vec::new();
    let mut call_names = Vec::new();
    for (i, closure) in lambda.closures.iter().enumerate() {
        let mut params = Vec::new();
        let mut param_types = Vec::new();
        for input in &closure.inputs {
            let Pat::Type(pat) = input else {
                return Err(Error::new_spanned(input, "expected a typed parameter"));
            };
            params.push(pat);
            param_types.push(&pat.ty);
        }
        let output = &closure.output;
        let body = &closure.body;
        let body = rewrite_captures(quote! { #body }, &captured, false)?;
        let (receiver, body) = if is_unsafe {
            (quote! { &mut self }, quote! { unsafe { #body } })
        } else {
            (quote! { &self }, body)
        };
        let call_name = if lambda.is_generic {
            Ident::new(&format!("call_{i}"), proc_macro2::Span::call_site())
        } else {
            Ident::new("call", proc_macro2::Span::call_site())
        };
        calls.push(quote! {
            #[allow(unused_unsafe, clippy::too_many_arguments)]
            fn #call_name(#receiver #(, #params)*) #output {
                #body
            }
        });
        sigs.push(quote! { fn(#(#param_types),*) #output });
        call_names.push((call_name, closure.inputs.len()));
    }
    // The copy and move constructors of the lambda copy and move each
    // capture, and its destructor destroys them in reverse order, as chosen
    // from their types by the traits in libcc2rs::__capture.
    let reversed: Vec<_> = names.iter().rev().collect();
    let (receiver, destroy, constructor, copy_from, move_from) = if is_unsafe {
        (
            quote! { &mut self },
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

    let lambda_struct = quote! {
        struct __Lambda {
            #(#names: #types,)*
        }
        impl __Lambda {
            #(#calls)*
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
    };
    let lambda_init = quote! { __Lambda { #(#names: #inits,)* } };

    if !lambda.is_generic {
        let sig = &sigs[0];
        return Ok(quote! {{
            #lambda_struct
            ::libcc2rs::FnPtr::<#sig>::#constructor(
                #lambda_init,
                __Lambda::call,
                __Lambda::copy_from,
                __Lambda::move_from,
                __Lambda::destroy,
            )
        }});
    }

    let mut specs = Vec::new();
    for (sig, (call_name, arity)) in sigs.iter().zip(&call_names) {
        let args: Vec<Ident> = (0..*arity)
            .map(|i| Ident::new(&format!("__a{i}"), proc_macro2::Span::call_site()))
            .collect();
        let call = if is_unsafe {
            quote! {
                |__l: &mut ::std::rc::Rc<::std::cell::UnsafeCell<__Lambda>> #(, #args)*| {
                    unsafe { &mut *__l.get() }.#call_name(#(#args),*)
                }
            }
        } else {
            quote! {
                |__l: &::std::rc::Rc<__Lambda> #(, #args)*| __l.#call_name(#(#args),*)
            }
        };
        specs.push(quote! {
            ::std::rc::Rc::new(::libcc2rs::FnPtr::<#sig>::#constructor(
                ::std::rc::Rc::clone(__l),
                #call,
                |__l| ::std::rc::Rc::clone(__l),
                |__l| ::std::rc::Rc::clone(__l),
                |_| {},
            )) as ::std::rc::Rc<dyn ::std::any::Any>
        });
    }
    let (shared, copy_from, move_from, destroy) = if is_unsafe {
        (
            quote! { ::std::rc::Rc::new(::std::cell::UnsafeCell::new(#lambda_init)) },
            quote! { |__l| ::std::rc::Rc::new(::std::cell::UnsafeCell::new(
                unsafe { &mut *__l.get() }.copy_from()
            )) },
            quote! { |__l| ::std::rc::Rc::new(::std::cell::UnsafeCell::new(
                unsafe { &mut *__l.get() }.move_from()
            )) },
            quote! { |__l| unsafe { &mut *__l.get() }.destroy() },
        )
    } else {
        (
            quote! { ::std::rc::Rc::new(#lambda_init) },
            quote! { |__l| ::std::rc::Rc::new(__l.copy_from()) },
            quote! { |__l| ::std::rc::Rc::new(__l.move_from()) },
            quote! { |__l| __l.destroy() },
        )
    };
    Ok(quote! {{
        #lambda_struct
        ::libcc2rs::FnPtr::<::libcc2rs::Generic>::from_generic_lambda(
            #shared,
            |__l| vec![#(#specs),*],
            #copy_from,
            #move_from,
            #destroy,
        )
    }})
}

pub fn expand(input: TokenStream, is_unsafe: bool) -> TokenStream {
    let lambda = parse_macro_input!(input as Lambda);
    expand_lambda(lambda, is_unsafe)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
