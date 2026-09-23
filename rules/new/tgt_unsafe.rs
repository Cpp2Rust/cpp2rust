// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: usize) -> *mut ::libc::c_void {
    libcc2rs::malloc_unsafe(a0)
}

unsafe fn f2(a0: *mut ::libc::c_void) {
    libcc2rs::free_unsafe(a0)
}

unsafe fn f3(a0: usize) -> *mut ::libc::c_void {
    libcc2rs::malloc_unsafe(a0)
}

unsafe fn f4(a0: *mut ::libc::c_void) {
    libcc2rs::free_unsafe(a0)
}

unsafe fn f5<T1>(a0: *mut T1) -> *mut T1 {
    a0
}
