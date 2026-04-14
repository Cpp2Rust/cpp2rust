// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

mod reinterpret;
pub use reinterpret::ByteRepr;

#[macro_export]
macro_rules! fn_ptr {
    ($f:expr, $ty:ty) => {
        $crate::FnPtr::new($f as $ty, $f as *const () as usize)
    };
}

// Lambda: no stable address, use 0. TODO: assign unique addr per lambda site so distinct
// lambdas don't compare equal.
#[macro_export]
macro_rules! fn_ptr_anon {
    ($f:expr, $ty:ty) => {
        $crate::FnPtr::new($f as $ty, 0)
    };
}

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

mod iterators;
pub use iterators::*;

mod compat;
pub use compat::*;

mod va_args;
pub use va_args::*;
