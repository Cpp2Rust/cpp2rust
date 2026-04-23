// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;

mod goto;
mod state_machine;
mod switch;

// switch!(match <condition> {
//     <pat> [if <guard>] => <body>,
//     <pat> [if <guard>] => <body>,
//     ...
//     _ => <body>,
// });

#[proc_macro]
pub fn switch(input: TokenStream) -> TokenStream {
    switch::expand(input)
}

#[proc_macro]
pub fn switch_break(_input: TokenStream) -> TokenStream {
    quote::quote! {
        compile_error!("switch_break!() can only be used inside switch!")
    }
    .into()
}

// goto_block! {
//     '<label> => <body>,
//     '<label> => <body>,
//     ...
// };

#[proc_macro]
pub fn goto_block(input: TokenStream) -> TokenStream {
    goto::expand(input)
}

#[proc_macro]
pub fn goto(_input: TokenStream) -> TokenStream {
    quote::quote! {
        compile_error!("goto!() can only be used inside goto_block!")
    }
    .into()
}
