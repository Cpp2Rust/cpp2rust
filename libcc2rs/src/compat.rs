// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use crate::rc::Ptr;
use crate::{AsPointer, Value};

unsafe extern "C" {
    #[cfg(target_os = "linux")]
    #[link_name = "malloc_usable_size"]
    fn platform_malloc_size(ptr: *mut c_void) -> usize;

    #[cfg(target_os = "macos")]
    #[link_name = "malloc_size"]
    fn platform_malloc_size(ptr: *const c_void) -> usize;

    #[cfg(target_os = "linux")]
    #[link_name = "__errno_location"]
    fn platform_errno_location() -> *mut i32;

    #[cfg(target_os = "macos")]
    #[link_name = "__error"]
    fn platform_errno_location() -> *mut i32;
}

/// # Safety
///
/// The pointer `ptr` must be a pointer to a block of memory allocated by
/// the appropriate allocator (e.g., `malloc`).
// The memory must not have been deallocated.
pub unsafe fn malloc_usable_size(ptr: *mut c_void) -> usize {
    #[cfg(target_os = "linux")]
    {
        unsafe { platform_malloc_size(ptr) }
    }
    #[cfg(target_os = "macos")]
    {
        unsafe { platform_malloc_size(ptr as *const c_void) }
    }
}

/// # Safety
///
/// Invokes the platform specific errno.
pub unsafe fn cpp2rust_errno_unsafe() -> *mut i32 {
    unsafe { platform_errno_location() }
}

thread_local! {
    static ERRNO: Value<i32> = Rc::new(RefCell::new(0));
}

pub fn cpp2rust_errno() -> Ptr<i32> {
    ERRNO.with(AsPointer::as_pointer)
}
