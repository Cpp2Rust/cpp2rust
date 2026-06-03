// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *const u8, a1: *const u8, a2: i32) -> i32 {
    libc::fnmatch(a0 as *const i8, a1 as *const i8, a2)
}
