// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let empty = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Unit => true,
            Fields::Named(f) => f.named.is_empty(),
            Fields::Unnamed(f) => f.unnamed.is_empty(),
        },
        _ => false,
    };
    if !empty {
        panic!("derive(ByteRepr) is only supported on empty structs, `{name}` is not empty");
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote::quote! {
        impl #impl_generics ::libcc2rs::ByteRepr for #name #ty_generics #where_clause {
            #[inline]
            fn byte_size() -> usize {
                1
            }
        }
    }
    .into()
}
