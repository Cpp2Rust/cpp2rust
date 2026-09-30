// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, LitInt, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let vis = &input.vis;
    let Data::Struct(data) = &input.data else {
        panic!("derive(Record) is only supported on structs, `{name}` is not one");
    };
    let mut fields: Vec<_> = match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.clone().unwrap();
                let offset: LitInt = field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("offset"))
                    .unwrap_or_else(|| panic!("field `{name}::{ident}` has no #[offset(...)]"))
                    .parse_args()
                    .unwrap();
                let offset: usize = offset.base10_parse().unwrap();
                (ident, offset)
            })
            .collect(),
        _ => Vec::new(),
    };
    let offsets = format_ident!("__{}_offsets", name);
    let idents: Vec<_> = fields.iter().map(|(ident, _)| ident.clone()).collect();
    let values: Vec<_> = fields.iter().map(|(_, offset)| *offset).collect();

    // The field that contains a byte is the last one that starts before it.
    fields.sort_by_key(|(_, offset)| std::cmp::Reverse(*offset));
    let locate = fields.iter().map(|(ident, offset)| {
        quote! {
            if offset >= #offset {
                return ::libcc2rs::__locate_field!(&self.#ident, offset - #offset, ty);
            }
        }
    });
    let locate_mut = fields.iter().map(|(ident, offset)| {
        quote! {
            if offset >= #offset {
                return ::libcc2rs::__locate_field_mut!(&mut self.#ident, offset - #offset, ty);
            }
        }
    });

    quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #vis struct #offsets {
            #(pub #idents: usize,)*
        }

        impl ::libcc2rs::Record for #name {
            type Offsets = #offsets;
            const OFFSETS: #offsets = #offsets { #(#idents: #values,)* };

            #[allow(unused_comparisons)]
            fn locate(
                &self,
                offset: usize,
                ty: ::std::any::TypeId,
            ) -> ::core::option::Option<&dyn ::std::any::Any> {
                if offset == 0 && ty == ::std::any::TypeId::of::<Self>() {
                    return ::core::option::Option::Some(self);
                }
                #(#locate)*
                ::core::option::Option::None
            }

            #[allow(unused_comparisons)]
            fn locate_mut(
                &mut self,
                offset: usize,
                ty: ::std::any::TypeId,
            ) -> ::core::option::Option<&mut dyn ::std::any::Any> {
                if offset == 0 && ty == ::std::any::TypeId::of::<Self>() {
                    return ::core::option::Option::Some(self);
                }
                #(#locate_mut)*
                ::core::option::Option::None
            }
        }
    }
    .into()
}
