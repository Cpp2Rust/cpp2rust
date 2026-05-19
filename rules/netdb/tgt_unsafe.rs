// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(
    a0: *const u8,
    a1: *const u8,
    a2: *const ::libc::addrinfo,
    a3: *mut *mut ::libc::addrinfo,
) -> i32 {
    libc::getaddrinfo(a0 as *const i8, a1 as *const i8, a2, a3)
}

unsafe fn f2(a0: *mut ::libc::addrinfo) {
    libc::freeaddrinfo(a0)
}
