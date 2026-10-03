// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

extern crate self as libcc2rs;

mod reinterpret;
pub use reinterpret::ByteRepr;

mod rc;
pub use rc::*;

mod field;
pub use field::{ElemPlace, FieldPlace, FieldPtr, Place, Record};
// Used by #[derive(Record)] and size_of_field!.
#[doc(hidden)]
pub mod __field {
    pub use crate::field::{
        Locate, LocateLeaf, LocateLeafMut, LocateMut, LocateRecord, LocateRecordMut,
        size_of_pointee,
    };
}

mod cstr;

mod void;
pub use void::*;

mod ptr_dyn;
pub use ptr_dyn::*;

include!(concat!(env!("OUT_DIR"), "/rule_shims.rs"));

mod fn_ptr_arg;
pub use fn_ptr_arg::{ArgRepr, FnPtrArg, record_from_repr};

mod fn_ptr;
pub use fn_ptr::FnPtr;

mod callable;
pub use callable::*;

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

mod fd;
pub use fd::*;

mod format;
pub use format::*;

pub use libcc2rs_macros::{
    ByteRepr, DeepClone, FnPtrArg, Record, VaArg, goto, goto_block, lambda, lambda_unsafe, switch,
};
