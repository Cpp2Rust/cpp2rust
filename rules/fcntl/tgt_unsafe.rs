// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe extern "C" {
    fn f1(a0: i32, a1: i32, ...) -> i32;
    fn f2(a0: *const i8, a1: i32, ...) -> i32;
}

unsafe fn f3() -> i32 {
    ::libc::O_CREAT
}

unsafe fn f4() -> i32 {
    ::libc::O_TRUNC
}

unsafe fn f5() -> i32 {
    ::libc::O_APPEND
}

unsafe fn f6() -> i32 {
    ::libc::O_EXCL
}

unsafe fn f7() -> i32 {
    ::libc::O_NONBLOCK
}

unsafe fn f8() -> i32 {
    ::libc::O_CLOEXEC
}

unsafe fn f9() -> i32 {
    ::libc::O_RDONLY
}

unsafe fn f10() -> i32 {
    ::libc::O_WRONLY
}

unsafe fn f11() -> i32 {
    ::libc::O_RDWR
}
