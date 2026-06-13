// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

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
pub unsafe fn strdup_unsafe(a0: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    unsafe { libc::strdup(a0) }
}
