// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> ::libc::tm {
    unsafe { std::mem::zeroed() }
}

fn t2() -> ::libc::timeval {
    unsafe { std::mem::zeroed() }
}

fn t3() -> ::libc::timespec {
    unsafe { std::mem::zeroed() }
}

unsafe fn f1(a0: *mut ::libc::time_t) -> ::libc::time_t {
    libc::time(a0)
}

unsafe fn f2(a0: ::libc::clockid_t, a1: *mut ::libc::timespec) -> i32 {
    libc::clock_gettime(a0, a1)
}

unsafe fn f4(a0: *const ::libc::time_t, a1: *mut ::libc::tm) -> *mut ::libc::tm {
    libc::gmtime_r(a0, a1)
}

unsafe fn f5(a0: *const ::libc::time_t, a1: *mut ::libc::tm) -> *mut ::libc::tm {
    libc::localtime_r(a0, a1)
}

unsafe fn f6(
    a0: *mut libc::c_char,
    a1: usize,
    a2: *const libc::c_char,
    a3: *const ::libc::tm,
) -> usize {
    libc::strftime(a0, a1, a2, a3)
}

unsafe fn f7(a0: *const libc::c_char, a1: *const ::libc::timeval) -> i32 {
    libc::utimes(a0, a1)
}

#[cfg(target_os = "linux")]
unsafe fn f8(a0: *mut libc::timeval, a1: *mut libc::timezone) -> i32 {
    libc::gettimeofday(a0, a1 as *mut libc::timezone)
}

#[cfg(target_os = "macos")]
unsafe fn f8(a0: *mut libc::timeval, a1: *mut libc::c_void) -> i32 {
    libc::gettimeofday(a0, a1)
}

unsafe fn f9() -> libc::clockid_t {
    libc::CLOCK_REALTIME
}
