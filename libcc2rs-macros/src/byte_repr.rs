// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Expr, Fields, parse_macro_input};

fn byte_size(attrs: &[Attribute]) -> Option<Expr> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("byte_size"))
        .map(|attr| attr.parse_args().unwrap())
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let Data::Struct(data) = &input.data else {
        panic!("derive(ByteRepr) is only supported on structs, `{name}` is not one");
    };
    let fields = match &data.fields {
        Fields::Named(fields) => fields.named.iter().collect(),
        Fields::Unit => Vec::new(),
        Fields::Unnamed(_) => {
            panic!("derive(ByteRepr) is not supported on tuple structs, `{name}` is one")
        }
    };

    let size = byte_size(&input.attrs)
        .unwrap_or_else(|| panic!("struct `{name}` has no #[byte_size(...)]"));

    // The bytes of each field are at its offset, and span the byte size of
    // its type, unless given by its byte_size attribute.
    let ranges: Vec<_> = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            let offset = crate::record::offset(name, field);
            let size = byte_size(&field.attrs)
                .map(|size| quote! { #size })
                .unwrap_or_else(|| quote! { <#ty as ::libcc2rs::ByteRepr>::byte_size() });
            quote! { (#offset)..(#offset) + #size }
        })
        .collect();
    let idents: Vec<_> = fields.iter().map(|field| &field.ident).collect();
    let types: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::libcc2rs::ByteRepr for #name #ty_generics #where_clause {
            #[inline]
            fn byte_size() -> usize {
                #size
            }

            #[allow(unused_variables)]
            fn to_bytes(&self, buf: &mut [u8]) {
                #(::libcc2rs::ByteRepr::to_bytes(&self.#idents, &mut buf[#ranges]);)*
            }

            #[allow(unused_variables)]
            fn from_bytes(buf: &[u8]) -> Self {
                Self {
                    #(#idents: <#types as ::libcc2rs::ByteRepr>::from_bytes(&buf[#ranges]),)*
                }
            }
        }
    }
    .into()
}
