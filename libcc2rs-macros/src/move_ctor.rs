// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn expand(input: TokenStream, is_unsafe: bool) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    if is_unsafe {
        quote! {
            impl #impl_generics ::libcc2rs::MoveCtorUnsafe for #name #ty_generics #where_clause {
                unsafe fn move_ctor(src: *mut Self) -> Self {
                    unsafe { Self::move_from(src) }
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics ::libcc2rs::MoveCtor for #name #ty_generics #where_clause {
                fn move_ctor(src: ::libcc2rs::Ptr<Self>) -> Self {
                    Self::move_from(src)
                }
            }
        }
    }
    .into()
}
