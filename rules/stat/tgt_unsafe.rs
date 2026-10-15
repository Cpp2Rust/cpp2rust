// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> ::libc::stat {
    unsafe { std::mem::zeroed() }
}

unsafe fn f1(a0: *const libc::c_char, a1: *mut ::libc::stat) -> i32 {
    libc::stat(a0, a1)
}

unsafe fn f2(a0: i32, a1: *mut ::libc::stat) -> i32 {
    libc::fstat(a0, a1)
}

unsafe fn f3(a0: *const libc::c_char, a1: ::libc::mode_t) -> i32 {
    libc::mkdir(a0, a1 as ::libc::mode_t)
}

unsafe fn f4(a0: *mut ::libc::stat) -> *mut ::libc::time_t {
    &raw mut (*a0).st_mtime
}
