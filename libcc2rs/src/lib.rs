// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

mod reinterpret;
pub use reinterpret::ByteRepr;

mod rc;
pub use rc::*;

mod libc_shims;
pub use libc_shims::*;

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

mod format;
pub use format::*;

pub use libcc2rs_macros::{goto, goto_block, switch};
