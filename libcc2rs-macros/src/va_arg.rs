// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote::quote! {
        impl #impl_generics ::core::convert::From<#name #ty_generics> for ::libcc2rs::VaArg
        #where_clause
        {
            fn from(v: #name #ty_generics) -> Self {
                ::libcc2rs::VaArg::Record(::std::rc::Rc::new(v))
            }
        }

        impl #impl_generics ::libcc2rs::VaArgGet for #name #ty_generics #where_clause {
            fn get(v: &::libcc2rs::VaArg) -> Self {
                ::libcc2rs::va_record_get(v)
            }
        }
    }
    .into()
}
