// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *const core::ffi::c_char, a1: *const core::ffi::c_char, a2: i32) -> i32 {
    libc::fnmatch(a0, a1, a2)
}
