// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

// Implements libcc2rs::CopyAssign or MoveAssign (or their unsafe versions)
// with the method of the same name, for `op` "copy" or "move".
pub fn expand(input: TokenStream, op: &str, is_unsafe: bool) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let method = format_ident!("{}_assign", op);
    let is_move = op == "move";
    if is_unsafe {
        let tr = if is_move {
            quote!(::libcc2rs::MoveAssignUnsafe)
        } else {
            quote!(::libcc2rs::CopyAssignUnsafe)
        };
        let src_ty = if is_move {
            quote!(*mut Self)
        } else {
            quote!(*const Self)
        };
        quote! {
            impl #impl_generics #tr for #name #ty_generics #where_clause {
                unsafe fn #method(dst: *mut Self, src: #src_ty) {
                    unsafe { (*dst).#method(src) };
                }
            }
        }
    } else {
        let tr = if is_move {
            quote!(::libcc2rs::MoveAssign)
        } else {
            quote!(::libcc2rs::CopyAssign)
        };
        quote! {
            impl #impl_generics #tr for #name #ty_generics #where_clause {
                fn #method(dst: &::libcc2rs::Ptr<Self>, src: ::libcc2rs::Ptr<Self>) {
                    dst.#method(src);
                }
            }
        }
    }
    .into()
}
