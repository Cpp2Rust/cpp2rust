// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::ffi::c_void;

extern "C" {
    #[cfg(target_os = "linux")]
    #[link_name = "malloc_usable_size"]
    fn platform_malloc_size(ptr: *mut c_void) -> usize;

    #[cfg(target_os = "macos")]
    #[link_name = "malloc_size"]
    fn platform_malloc_size(ptr: *const c_void) -> usize;
}

/// # Safety
///
/// The pointer `ptr` must be a pointer to a block of memory allocated by
/// the appropriate allocator (e.g., `malloc`).
// The memory must not have been deallocated.
pub unsafe fn malloc_usable_size(ptr: *mut c_void) -> usize {
    #[cfg(target_os = "linux")]
    {
        platform_malloc_size(ptr)
    }
    #[cfg(target_os = "macos")]
    {
        platform_malloc_size(ptr as *const c_void)
    }
}
