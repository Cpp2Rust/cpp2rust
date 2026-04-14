// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

mod reinterpret;
pub use reinterpret::ByteRepr;

#[macro_export]
macro_rules! fn_ptr {
    ($f:expr, $ty:ty) => {
        $crate::Ptr::from_fn($f as $ty, $f as *const () as usize)
    };
}

mod rc;
pub use rc::*;

mod inc;
pub use inc::*;

mod dec;
pub use dec::*;

mod rules;
pub use rules::*;

mod io;
pub use io::*;

mod iterators;
pub use iterators::*;

mod compat;
pub use compat::*;

mod va_args;
pub use va_args::*;
