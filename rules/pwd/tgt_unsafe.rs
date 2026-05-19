// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: u32) -> *mut ::libc::passwd {
    libc::getpwuid(a0)
}
