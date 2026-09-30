// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote::quote! {
        impl #impl_generics ::libcc2rs::FnPtrArg for #name #ty_generics #where_clause {
            #[inline]
            fn to_repr(&self) -> ::libcc2rs::ArgRepr<'_> {
                ::libcc2rs::ArgRepr::Record(self)
            }
            fn from_repr(r: &::libcc2rs::ArgRepr) -> Self {
                ::libcc2rs::record_from_repr(r)
            }
        }
    }
    .into()
}
