// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(
    a0: i32,
    a1: *mut ::libc::fd_set,
    a2: *mut ::libc::fd_set,
    a3: *mut ::libc::fd_set,
    a4: *mut ::libc::timeval,
) -> i32 {
    libc::select(a0, a1, a2, a3, a4)
}
