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
            impl #impl_generics ::libcc2rs::DestructorUnsafe for #name #ty_generics #where_clause {
                unsafe fn destroy(p: *mut Self) {
                    unsafe { (*p).destructor() }
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics ::libcc2rs::Destructor for #name #ty_generics #where_clause {
                fn destroy(p: ::libcc2rs::Ptr<Self>) {
                    p.destructor()
                }
            }
        }
    }
    .into()
}
