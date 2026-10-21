// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> Option<unsafe fn(i32)> {
    None
}

unsafe fn f1(a0: i32, a1: *const ::libc::sigaction, a2: *mut ::libc::sigaction) -> i32 {
    libc::sigaction(a0, a1, a2)
}
