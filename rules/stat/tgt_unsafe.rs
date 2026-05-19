// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *const u8, a1: *mut ::libc::stat) -> i32 {
    libc::stat(a0 as *const i8, a1)
}

unsafe fn f2(a0: i32, a1: *mut ::libc::stat) -> i32 {
    libc::fstat(a0, a1)
}

unsafe fn f3(a0: *const u8, a1: u32) -> i32 {
    libc::mkdir(a0 as *const i8, a1)
}
