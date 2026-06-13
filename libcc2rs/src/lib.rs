// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

mod reinterpret;
pub use reinterpret::ByteRepr;

mod rc;
pub use rc::*;

mod fn_ptr;
pub use fn_ptr::FnPtr;

mod inc;
pub use inc::*;

mod dec;
pub use dec::*;

mod rules;
pub use rules::*;

mod io;
pub use io::*;

mod alloc;
pub use alloc::*;

mod iterators;
pub use iterators::*;

mod compat;
pub use compat::*;

mod va_args;
pub use va_args::*;

pub use libcc2rs_macros::{goto, goto_block, switch};

pub const fn char_array<const N: usize>(s: &[u8; N]) -> [core::ffi::c_char; N] {
    let mut out = [0 as core::ffi::c_char; N];
    let mut i = 0;
    while i < N {
        out[i] = s[i] as core::ffi::c_char;
        i += 1;
    }
    out
}
