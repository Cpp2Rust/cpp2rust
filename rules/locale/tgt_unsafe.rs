// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32, a1: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    libc::setlocale(a0, a1)
}
