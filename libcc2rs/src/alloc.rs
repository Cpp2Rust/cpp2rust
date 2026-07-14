// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::rc::{AnyPtr, Ptr};

pub fn malloc_refcount(a0: usize) -> AnyPtr {
    Ptr::alloc_array(vec![0u8; a0].into_boxed_slice()).to_any()
}

pub fn free_refcount(a0: AnyPtr) {
    if a0.is_null() {
        return;
    }
    a0.reinterpret_cast::<u8>().delete_array();
}

pub fn realloc_refcount(a0: AnyPtr, a1: usize) -> AnyPtr {
    if a0.is_null() {
        return malloc_refcount(a1);
    }
    let __new = Ptr::alloc_array(vec![0u8; a1].into_boxed_slice()).to_any();
    let __n = a1.min(a0.reinterpret_cast::<u8>().len());
    __new.memcpy(&a0, __n);
    a0.reinterpret_cast::<u8>().delete_array();
    __new
}

pub fn calloc_refcount(a0: usize, a1: usize) -> AnyPtr {
    Ptr::alloc_array(vec![0u8; a0.wrapping_mul(a1)].into_boxed_slice()).to_any()
}

pub fn strdup_refcount(a0: Ptr<u8>) -> Ptr<u8> {
    let mut bytes: Vec<u8> = a0.to_c_string_iterator().collect();
    bytes.push(0);
    Ptr::alloc_array(bytes.into_boxed_slice())
}

/// # Safety
///
/// Same contract as C's `malloc`.
pub unsafe fn malloc_unsafe(a0: usize) -> *mut libc::c_void {
    unsafe { libc::malloc(a0) }
}

/// # Safety
///
/// Same contract as C's `free`.
pub unsafe fn free_unsafe(a0: *mut libc::c_void) {
    unsafe { libc::free(a0) }
}

/// # Safety
///
/// Same contract as C's `realloc`.
pub unsafe fn realloc_unsafe(a0: *mut libc::c_void, a1: usize) -> *mut libc::c_void {
    unsafe { libc::realloc(a0, a1) }
}

/// # Safety
///
/// Same contract as C's `calloc`.
pub unsafe fn calloc_unsafe(a0: usize, a1: usize) -> *mut libc::c_void {
    unsafe { libc::calloc(a0, a1) }
}

/// # Safety
///
/// Same contract as C's `strdup`.
pub unsafe fn strdup_unsafe(a0: *const libc::c_char) -> *mut libc::c_char {
    unsafe { libc::strdup(a0) }
}
