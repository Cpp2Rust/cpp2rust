// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32) -> i32 {
    libc::close(a0)
}

unsafe fn f2(a0: i32, a1: i64, a2: i32) -> i64 {
    libc::lseek(a0, a1, a2)
}

unsafe fn f3(a0: i32, a1: *mut ::libc::c_void, a2: usize) -> isize {
    libc::read(a0, a1, a2)
}

unsafe fn f4(a0: *const core::ffi::c_char) -> i32 {
    libc::unlink(a0)
}

unsafe fn f5(a0: *mut i32) -> i32 {
    libc::pipe(a0)
}

unsafe fn f6(a0: i32, a1: i64) -> i32 {
    libc::ftruncate(a0, a1)
}

unsafe fn f7(a0: i32) -> i32 {
    libc::isatty(a0)
}

unsafe fn f8() -> u32 {
    libc::geteuid()
}

unsafe fn f9(a0: *mut core::ffi::c_char, a1: usize) -> i32 {
    libc::gethostname(a0, a1)
}

unsafe fn f10(a0: i32, a1: *const ::libc::c_void, a2: usize) -> isize {
    libc::write(a0, a1, a2)
}

unsafe fn f11(a0: *const core::ffi::c_char) -> i32 {
    libc::rmdir(a0)
}
