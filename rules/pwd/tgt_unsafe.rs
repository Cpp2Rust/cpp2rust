// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: u32) -> *mut ::libc::passwd {
    libc::getpwuid(a0)
}

unsafe fn f2(
    a0: u32,
    a1: *mut ::libc::passwd,
    a2: *mut u8,
    a3: usize,
    a4: *mut *mut ::libc::passwd,
) -> i32 {
    libc::getpwuid_r(a0, a1, a2 as *mut i8, a3, a4)
}
