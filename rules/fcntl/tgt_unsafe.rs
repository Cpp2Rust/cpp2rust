// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe extern "C" {
    fn f1(a0: i32, a1: i32, ...) -> i32;
    fn f2(a0: *const i8, a1: i32, ...) -> i32;
}

fn f3() -> i32 {
    ::libc::O_CREAT
}

fn f4() -> i32 {
    ::libc::O_TRUNC
}

fn f5() -> i32 {
    ::libc::O_APPEND
}

fn f6() -> i32 {
    ::libc::O_EXCL
}

fn f7() -> i32 {
    ::libc::O_NONBLOCK
}

fn f8() -> i32 {
    ::libc::O_CLOEXEC
}

fn f9() -> i32 {
    ::libc::O_RDONLY
}

fn f10() -> i32 {
    ::libc::O_WRONLY
}

fn f11() -> i32 {
    ::libc::O_RDWR
}
