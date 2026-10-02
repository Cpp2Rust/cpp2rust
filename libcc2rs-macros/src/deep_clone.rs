// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Type, parse_macro_input};

// Whether `ty` is a Value<_>, which is copied with DeepClone.
fn is_value(ty: &Type) -> bool {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "Value"),
        _ => false,
    }
}

fn clone_field(ty: &Type, field: TokenStream2) -> TokenStream2 {
    if is_value(ty) {
        quote! { ::libcc2rs::DeepClone::deep_clone(&self.#field) }
    } else {
        quote! { ::core::clone::Clone::clone(&self.#field) }
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let Data::Struct(data) = &input.data else {
        panic!("derive(DeepClone) is only supported on structs, `{name}` is not one");
    };
    let body = match &data.fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|field| {
                let ident = field.ident.as_ref().unwrap();
                let value = clone_field(&field.ty, quote! { #ident });
                quote! { #ident: #value }
            });
            quote! { Self { #(#fields,)* } }
        }
        Fields::Unnamed(fields) => {
            let fields = fields.unnamed.iter().enumerate().map(|(i, field)| {
                let index = Index::from(i);
                clone_field(&field.ty, quote! { #index })
            });
            quote! { Self(#(#fields,)*) }
        }
        Fields::Unit => quote! { Self },
    };
    quote! {
        impl #impl_generics ::core::clone::Clone for #name #ty_generics #where_clause {
            fn clone(&self) -> Self {
                #body
            }
        }
    }
    .into()
}
