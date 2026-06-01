// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn t1() -> ::libc::fd_set {
    std::mem::zeroed::<::libc::fd_set>()
}

unsafe fn f1(
    a0: i32,
    a1: *mut ::libc::fd_set,
    a2: *mut ::libc::fd_set,
    a3: *mut ::libc::fd_set,
    a4: *mut ::libc::timeval,
) -> i32 {
    libc::select(a0, a1, a2, a3, a4)
}

unsafe fn f2(a0: i32, a1: *mut ::libc::fd_set) {
    libc::FD_SET(a0, a1);
}
unsafe fn f3(a0: i32, a1: *mut ::libc::fd_set) {
    libc::FD_CLR(a0, a1);
}
unsafe fn f4(a0: i32, a1: *const ::libc::fd_set) -> i32 {
    libc::FD_ISSET(a0, a1) as i32
}
unsafe fn f5(a0: *mut ::libc::fd_set) {
    libc::FD_ZERO(a0);
}
