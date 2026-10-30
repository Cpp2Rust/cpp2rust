// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;

mod assign;
mod byte_repr;
mod cpp_from;
mod destructor;
mod fn_ptr_arg;
mod goto;
mod lambda;
mod move_ctor;
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
pub fn lambda(input: TokenStream) -> TokenStream {
    lambda::expand(input, false)
}

#[proc_macro]
pub fn lambda_unsafe(input: TokenStream) -> TokenStream {
    lambda::expand(input, true)
}

#[proc_macro]
pub fn goto(_input: TokenStream) -> TokenStream {
    quote::quote! {
        compile_error!("goto!() can only be used inside goto_block!")
    }
    .into()
}

//     #[derive(ByteRepr)]
//     #[byte_size(16)]
//     pub struct S {
//         #[offset(0)]
//         pub x: i32,
//         #[offset(4)]
//         #[byte_size(12)]
//         pub a: Box<[i32]>,
//         ...
//     }
//
// Implements libcc2rs::ByteRepr for S, laying out each field at the byte
// offset given by its offset attribute (see derive(Record)). The byte_size
// attribute gives the byte size of S in C, and of the fields whose size in C
// differs from the byte_size() of their type (e.g., arrays and pointers).

#[proc_macro_derive(ByteRepr, attributes(offset, byte_size))]
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

//     #[derive(MoveCtor)]
//     pub struct S { ... }
//     impl S {
//         pub fn move_from(o: Ptr<S>) -> Self { ... }
//     }
//
// Implements libcc2rs::MoveCtor for S with its move constructor, move_from,
// such that lambda! moves the captures of type S with it. MoveCtorUnsafe is
// the same for the unsafe model, whose move_from takes a *mut S.

#[proc_macro_derive(MoveCtor)]
pub fn derive_move_ctor(input: TokenStream) -> TokenStream {
    move_ctor::expand(input, false)
}

#[proc_macro_derive(MoveCtorUnsafe)]
pub fn derive_move_ctor_unsafe(input: TokenStream) -> TokenStream {
    move_ctor::expand(input, true)
}

//     #[derive(CopyAssign, MoveAssign)]
//     pub struct S { ... }
//     impl SImpl for Ptr<S> {
//         fn copy_assign(&self, o: Ptr<S>) -> Ptr<S> { ... }
//         fn move_assign(&self, o: Ptr<S>) -> Ptr<S> { ... }
//     }
//
// Implements libcc2rs::CopyAssign and MoveAssign for S with its copy and move
// assignment operators, such that std::swap assigns the values of type S
// with them, as C++ does. CopyAssignUnsafe and MoveAssignUnsafe are the same
// for the unsafe model, whose copy_assign and move_assign are methods of S.

#[proc_macro_derive(CopyAssign)]
pub fn derive_copy_assign(input: TokenStream) -> TokenStream {
    assign::expand(input, "copy", false)
}

#[proc_macro_derive(CopyAssignUnsafe)]
pub fn derive_copy_assign_unsafe(input: TokenStream) -> TokenStream {
    assign::expand(input, "copy", true)
}

#[proc_macro_derive(MoveAssign)]
pub fn derive_move_assign(input: TokenStream) -> TokenStream {
    assign::expand(input, "move", false)
}

#[proc_macro_derive(MoveAssignUnsafe)]
pub fn derive_move_assign_unsafe(input: TokenStream) -> TokenStream {
    assign::expand(input, "move", true)
}

//     #[derive(Destructor)]
//     pub struct S { ... }
//     impl SImpl for Ptr<S> {
//         fn destructor(&self) { ... }
//     }
//
// Implements libcc2rs::Destructor for S with its destructor, such that
// lambda! destroys the captures of type S with it. DestructorUnsafe is the
// same for the unsafe model, whose destructor is a method of S.

#[proc_macro_derive(Destructor)]
pub fn derive_destructor(input: TokenStream) -> TokenStream {
    destructor::expand(input, false)
}

#[proc_macro_derive(DestructorUnsafe)]
pub fn derive_destructor_unsafe(input: TokenStream) -> TokenStream {
    destructor::expand(input, true)
}

//     #[derive(CppFrom)]
//     #[cpp_from(i32, new, value)]
//     #[cpp_into(i32, to_i32)]
//     pub struct S { ... }
//
// Encodes the C++ converting construction and conversion operators in rules.

#[proc_macro_derive(CppFrom, attributes(cpp_from, cpp_from_move, cpp_into))]
pub fn derive_cpp_from(input: TokenStream) -> TokenStream {
    cpp_from::expand(input, false)
}

#[proc_macro_derive(CppFromUnsafe, attributes(cpp_from, cpp_from_move, cpp_into))]
pub fn derive_cpp_from_unsafe(input: TokenStream) -> TokenStream {
    cpp_from::expand(input, true)
}
