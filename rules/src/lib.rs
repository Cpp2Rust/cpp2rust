// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#![allow(warnings)]
mod modules;
pub use modules::*;

pub fn fread_refcount(
    a0: ::libcc2rs::AnyPtr,
    a1: u64,
    a2: u64,
    a3: ::libcc2rs::Ptr<::std::fs::File>,
) -> u64 {
    modules::stdio_tgt_refcount::f5(a0, a1, a2, a3)
}

pub unsafe fn fread_unsafe(
    a0: *mut ::libc::c_void,
    a1: u64,
    a2: u64,
    a3: *mut ::std::fs::File,
) -> u64 {
    unsafe { modules::stdio_tgt_unsafe::f5(a0, a1, a2, a3) }
}
