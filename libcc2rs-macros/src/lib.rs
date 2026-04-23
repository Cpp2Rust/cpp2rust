// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;

mod goto;
mod state_machine;
mod switch;

//     switch!(match <condition> {
//         <pat> [if <guard>] => { /* body; may contain break or continue */ },
//         ...
//         _ => <body>,
//     });
//
// Desugars to a goto_block! with a synthetic dispatch arm prepended.
//
//     goto_block! {
//         '__dispatch => {
//             match <condition> {
//                 <pat_1> => { __s = 1; continue '__sm; }
//                 ...
//                 _       => break '__sm,
//             }
//         },
//         '__c1 => { /* body_1 with `break` rewritten to `break '__sm` */ },
//         ...
//         '__cN => { /* body_N with same rewrite */ },
//     };
//
// __sm is the inner label used to describe the state machine insinde goto_block. See goto_block!
// for more info.

#[proc_macro]
pub fn switch(input: TokenStream) -> TokenStream {
    switch::expand(input)
}

//     goto_block! {
//         '<label> => { /* body; may contain `break` or `continue` */ },
//         ...
//     };
//
// Expands to
//
//     {
//         let mut __user_break:    bool = false; // only if any arm has `break`
//         let mut __user_continue: bool = false; // only if any arm has `continue`
//         let mut __s: u32 = 0;
//         '__sm: loop {
//             match __s {
//                 0u32 => {
//                     /* body_0 with these rewrites (outside nested user loops): */
//                     /*   break;    ->  { __user_break    = true; break '__sm; } */
//                     /*   continue; ->  { __user_continue = true; break '__sm; } */
//                     __s = 1; continue '__sm;
//                 }
//                 ...
//                 (N-1)u32 => { /* body_N-1 with same rewrites */ break '__sm; }
//                 _        => break '__sm, // written only for match exhaustiveness
//             }
//         }
//         if __user_break    { break; }    // only if any arm has `break;`
//         if __user_continue { continue; } // only if any arm has `continue;`
//     }
//
//  __user_break and __user_continue propagate the `break`s and `continue`s outside the goto state
//  machine loop.

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
