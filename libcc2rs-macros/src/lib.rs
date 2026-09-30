// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;

mod byte_repr;
mod fn_ptr_arg;
mod goto;
mod record;
mod state_machine;
mod switch;
mod va_arg;

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

//     goto_block!({
//         '<label>: { /* body; may contain `break`, `continue`, or goto!('other) */ }
//         ...
//     });
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

//     #[derive(ByteRepr)]
//     pub struct S;
//
// Adds ByteRepr implementation for S. Currently only empty structs are handled. Non-empty structs
// panic.

#[proc_macro_derive(ByteRepr)]
pub fn derive_byte_repr(input: TokenStream) -> TokenStream {
    byte_repr::expand(input)
}

#[proc_macro_derive(VaArg)]
pub fn derive_va_arg(input: TokenStream) -> TokenStream {
    va_arg::expand(input)
}

#[proc_macro_derive(FnPtrArg)]
pub fn derive_fn_ptr_arg(input: TokenStream) -> TokenStream {
    fn_ptr_arg::expand(input)
}

//     #[derive(Record)]
//     pub struct S {
//         #[offset(0)]
//         pub x: i32,
//         ...
//     }
//
// Implements libcc2rs::Record for S, which lets pointers to the fields of S
// be created by field_ptr!. The argument of the offset attribute is the byte
// offset of the field in the C layout of S, as a constant expression (e.g.,
// `offset_of!(libc::stat, st_size)`).

#[proc_macro_derive(Record, attributes(offset))]
pub fn derive_record(input: TokenStream) -> TokenStream {
    record::expand(input)
}
