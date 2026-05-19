// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32, a1: *const u8) -> *mut u8 {
    libc::setlocale(a0, a1 as *const i8) as *mut u8
}
