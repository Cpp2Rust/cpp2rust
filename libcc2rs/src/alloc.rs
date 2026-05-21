// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

/// # Safety
///
/// Same contract as C's `malloc`.
pub unsafe fn malloc_unsafe(a0: u64) -> *mut libc::c_void {
    unsafe { libc::malloc(a0 as libc::size_t) }
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
pub unsafe fn realloc_unsafe(a0: *mut libc::c_void, a1: u64) -> *mut libc::c_void {
    unsafe { libc::realloc(a0, a1 as libc::size_t) }
}

/// # Safety
///
/// Same contract as C's `calloc`.
pub unsafe fn calloc_unsafe(a0: u64, a1: u64) -> *mut libc::c_void {
    unsafe { libc::calloc(a0 as libc::size_t, a1 as libc::size_t) }
}

/// # Safety
///
/// Same contract as C's `strdup`.
pub unsafe fn strdup_unsafe(a0: *const u8) -> *mut u8 {
    unsafe { libc::strdup(a0 as *const libc::c_char) as *mut u8 }
}
