// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32, a1: i32, a2: *const ::libc::termios) -> i32 {
    libc::tcsetattr(a0, a1, a2)
}

unsafe fn f2(a0: i32, a1: *mut ::libc::termios) -> i32 {
    libc::tcgetattr(a0, a1)
}
