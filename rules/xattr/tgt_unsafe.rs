// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#[cfg(target_os = "linux")]
unsafe fn f1(
    a0: i32,
    a1: *const libc::c_char,
    a2: *const ::libc::c_void,
    a3: usize,
    a4: i32,
) -> i32 {
    libc::fsetxattr(a0, a1, a2, a3, a4)
}
