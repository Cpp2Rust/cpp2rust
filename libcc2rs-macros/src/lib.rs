// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;

mod goto;
mod state_machine;
mod switch;

#[proc_macro]
pub fn switch(input: TokenStream) -> TokenStream {
    switch::expand(input)
}

#[proc_macro]
pub fn goto_block(input: TokenStream) -> TokenStream {
    goto::expand(input)
}

#[proc_macro]
pub fn goto(_input: TokenStream) -> TokenStream {
    todo!();
}
