// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Ident, Token, Type, parse_macro_input};

//     #[derive(CppFrom)]
//     #[cpp_from(i32, new, value)]        // S(int)
//     #[cpp_from(i32, new_1, some_value)] // S(int = 0)
//     #[cpp_from(T, new_2, ptr)]          // S(const T &)
//     #[cpp_from(T, new_3, mut_ptr)]      // S(T &)
//     #[cpp_from_move(T, new_4, ptr)]     // S(T &&)
//     #[cpp_into(i32, to_i32)]            // operator int() const
//     pub struct S { ... }
//
// Implements libcc2rs::CppFrom<Source> for S with each converting constructor
// of S, and libcc2rs::CppFrom<S> for each type S converts to with a
// conversion operator, such that the rules convert values as C++ does
// implicitly. `value` passes the pointee to the constructor, `ptr` the
// pointer, `mut_ptr` a mutable pointer, and `some_` wraps the argument in
// Some for a defaulted parameter. CppFromUnsafe is the same for the unsafe
// model.

#[derive(Clone, Copy)]
enum Kind {
    Value,
    Ptr,
    MutPtr,
}

struct Conversion {
    ty: Type,
    method: Ident,
    kind: Kind,
    some: bool,
}

impl Parse for Conversion {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = input.parse()?;
        input.parse::<Token![,]>()?;
        let method = input.parse()?;
        let mut kind = Kind::Ptr;
        let mut some = false;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let spelled: Ident = input.parse()?;
            let name = spelled.to_string();
            let name = match name.strip_prefix("some_") {
                Some(rest) => {
                    some = true;
                    rest
                }
                None => name.as_str(),
            };
            kind = match name {
                "value" => Kind::Value,
                "ptr" => Kind::Ptr,
                "mut_ptr" => Kind::MutPtr,
                _ => {
                    return Err(syn::Error::new(
                        spelled.span(),
                        "expected `value`, `ptr` or `mut_ptr`, optionally prefixed by `some_`",
                    ));
                }
            };
        }
        Ok(Conversion {
            ty,
            method,
            kind,
            some,
        })
    }
}

#[derive(Default)]
struct Source {
    from: Option<Conversion>,
    from_move: Option<Conversion>,
}

fn ctor_call(
    name: &Ident,
    conversion: &Conversion,
    is_unsafe: bool,
    is_move: bool,
) -> TokenStream2 {
    let method = &conversion.method;
    let arg = match (conversion.kind, is_unsafe) {
        (Kind::Value, true) => quote!((*src).clone()),
        (Kind::Value, false) => quote!(src.read()),
        (Kind::MutPtr, true) if !is_move => quote!(src.cast_mut()),
        (Kind::Ptr | Kind::MutPtr, _) => quote!(src),
    };
    let arg = if conversion.some {
        quote!(Some(#arg))
    } else {
        arg
    };
    quote!(#name::#method(#arg))
}

fn method(is_unsafe: bool, signature: TokenStream2, body: TokenStream2) -> TokenStream2 {
    if is_unsafe {
        quote! { unsafe #signature { unsafe { #body } } }
    } else {
        quote! { #signature { #body } }
    }
}

pub fn expand(input: TokenStream, is_unsafe: bool) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut sources: Vec<(String, Source)> = Vec::new();
    let mut intos: Vec<Conversion> = Vec::new();
    for attr in &input.attrs {
        let Some(attr_name) = attr.path().get_ident().map(ToString::to_string) else {
            continue;
        };
        let is_move = match attr_name.as_str() {
            "cpp_from" => false,
            "cpp_from_move" => true,
            "cpp_into" => {
                intos.push(attr.parse_args().expect("cpp_into(Type, method)"));
                continue;
            }
            _ => continue,
        };
        let conversion: Conversion = attr
            .parse_args()
            .expect("cpp_from(Type, method, value|ptr|mut_ptr)");
        let ty = &conversion.ty;
        let key = quote!(#ty).to_string();
        let source = match sources.iter().position(|(k, _)| *k == key) {
            Some(i) => &mut sources[i].1,
            None => {
                sources.push((key.clone(), Source::default()));
                &mut sources.last_mut().unwrap().1
            }
        };
        let slot = if is_move {
            &mut source.from_move
        } else {
            &mut source.from
        };
        assert!(slot.is_none(), "two conversions from {key}");
        *slot = Some(conversion);
    }

    let tr = if is_unsafe {
        quote!(::libcc2rs::CppFromUnsafe)
    } else {
        quote!(::libcc2rs::CppFrom)
    };
    let pointer = |ty: &Type, is_mut: bool| {
        if is_unsafe && is_mut {
            quote!(*mut #ty)
        } else if is_unsafe {
            quote!(*const #ty)
        } else {
            quote!(::libcc2rs::Ptr<#ty>)
        }
    };

    let from_impls = sources.iter().map(|(_, source)| {
        let any = source.from.as_ref().or(source.from_move.as_ref()).unwrap();
        let ty = &any.ty;
        let src = pointer(ty, false);
        let from_body = match &source.from {
            Some(conversion) => ctor_call(name, conversion, is_unsafe, false),
            None => quote!(unreachable!("no conversion from an lvalue")),
        };
        let from_fn = method(is_unsafe, quote!(fn cpp_from(src: #src) -> Self), from_body);
        let move_fn = source.from_move.as_ref().map(|conversion| {
            let src = pointer(ty, true);
            method(
                is_unsafe,
                quote!(fn cpp_from_move(src: #src) -> Self),
                ctor_call(name, conversion, is_unsafe, true),
            )
        });
        quote! {
            impl #impl_generics #tr<#ty> for #name #ty_generics #where_clause {
                #from_fn
                #move_fn
            }
        }
    });

    let impl_trait = format_ident!("{}Impl", name);
    let into_impls = intos.iter().map(|conversion| {
        let ty = &conversion.ty;
        let op = &conversion.method;
        let self_ty = quote!(#name #ty_generics);
        let src = pointer(&syn::parse_quote!(#self_ty), false);
        let body = if is_unsafe {
            quote!((*src).#op())
        } else {
            quote!(#impl_trait::#op(&src))
        };
        let from_fn = method(is_unsafe, quote!(fn cpp_from(src: #src) -> Self), body);
        quote! {
            impl #impl_generics #tr<#self_ty> for #ty #where_clause {
                #from_fn
            }
        }
    });

    quote! {
        #(#from_impls)*
        #(#into_impls)*
    }
    .into()
}
