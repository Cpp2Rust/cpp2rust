extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_time_0() {
    let mut t1: ::libc::time_t = libc::time(std::ptr::null_mut());
    let mut t2: ::libc::time_t = (0 as ::libc::time_t);
    let mut t3: ::libc::time_t = libc::time((&mut t2 as *mut ::libc::time_t));
    assert!(((((t1) > (1500000000 as ::libc::time_t)) as i32) != 0));
    assert!(((((t2) == (t3)) as i32) != 0));
    assert!(((((t3) >= (t1)) as i32) != 0));
}
pub unsafe fn print_tm_1(mut t: ::libc::time_t) {
    let mut tm: ::libc::tm = unsafe { std::mem::zeroed() };
    assert!(
        (((!((libc::gmtime_r(
            (&mut t as *mut ::libc::time_t).cast_const(),
            (&mut tm as *mut ::libc::tm)
        ))
        .is_null())) as i32)
            != 0)
    );
    (unsafe {
        libc::printf(
            (c"%d-%d-%d %d:%d:%d wday=%d yday=%d %s gmtoff=%ld isdst=%d\n"
                .as_ptr()
                .cast_mut())
            .cast_const() as *const libc::c_char,
            (tm.tm_year),
            (tm.tm_mon),
            (tm.tm_mday),
            (tm.tm_hour),
            (tm.tm_min),
            (tm.tm_sec),
            (tm.tm_wday),
            (tm.tm_yday),
            (tm.tm_zone),
            (tm.tm_gmtoff),
            (tm.tm_isdst),
        )
    });
}
pub unsafe fn test_gmtime_r_2() {
    (unsafe { print_tm_1((0 as ::libc::time_t)) });
    (unsafe { print_tm_1((1 as ::libc::time_t)) });
    (unsafe { print_tm_1((86399 as ::libc::time_t)) });
    (unsafe { print_tm_1((86400 as ::libc::time_t)) });
    (unsafe { print_tm_1((951782400 as ::libc::time_t)) });
    (unsafe { print_tm_1((951868799 as ::libc::time_t)) });
    (unsafe { print_tm_1((1704067199 as ::libc::time_t)) });
    (unsafe { print_tm_1((1704067200 as ::libc::time_t)) });
    (unsafe { print_tm_1((1721126096 as ::libc::time_t)) });
    (unsafe { print_tm_1((4102444800_i64 as ::libc::time_t)) });
}
pub unsafe fn print_local_tm_3(mut t: ::libc::time_t) {
    let mut tm: ::libc::tm = unsafe { std::mem::zeroed() };
    assert!(
        (((!((libc::localtime_r(
            (&mut t as *mut ::libc::time_t).cast_const(),
            (&mut tm as *mut ::libc::tm)
        ))
        .is_null())) as i32)
            != 0)
    );
    (unsafe {
        libc::printf(
            (c"%d-%d-%d %d:%d:%d wday=%d yday=%d %s gmtoff=%ld isdst=%d\n"
                .as_ptr()
                .cast_mut())
            .cast_const() as *const libc::c_char,
            (tm.tm_year),
            (tm.tm_mon),
            (tm.tm_mday),
            (tm.tm_hour),
            (tm.tm_min),
            (tm.tm_sec),
            (tm.tm_wday),
            (tm.tm_yday),
            (tm.tm_zone),
            (tm.tm_gmtoff),
            (tm.tm_isdst),
        )
    });
}
pub unsafe fn test_localtime_r_4() {
    (unsafe { print_local_tm_3((0 as ::libc::time_t)) });
    (unsafe { print_local_tm_3((951782400 as ::libc::time_t)) });
    (unsafe { print_local_tm_3((1704067199 as ::libc::time_t)) });
    (unsafe { print_local_tm_3((1721126096 as ::libc::time_t)) });
    (unsafe { print_local_tm_3((1735689600 as ::libc::time_t)) });
}
pub unsafe fn test_strftime_5() {
    let mut t: ::libc::time_t = (1721126096 as ::libc::time_t);
    let mut tm: ::libc::tm = unsafe { std::mem::zeroed() };
    assert!(
        (((!((libc::gmtime_r(
            (&mut t as *mut ::libc::time_t).cast_const(),
            (&mut tm as *mut ::libc::tm)
        ))
        .is_null())) as i32)
            != 0)
    );
    let mut buf: [libc::c_char; 64] = [(0 as libc::c_char); 64];
    assert!(
        ((((libc::strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>(),
            (c"%Y-%m-%d %H:%M:%S".as_ptr().cast_mut()).cast_const(),
            (&mut tm as *mut ::libc::tm).cast_const()
        )) > (0_usize)) as i32)
            != 0)
    );
    (unsafe {
        libc::printf(
            (c"%s\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (buf.as_mut_ptr()),
        )
    });
    assert!(
        ((((libc::strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>(),
            (c"%a, %d %b %Y %T".as_ptr().cast_mut()).cast_const(),
            (&mut tm as *mut ::libc::tm).cast_const()
        )) > (0_usize)) as i32)
            != 0)
    );
    (unsafe {
        libc::printf(
            (c"%s\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (buf.as_mut_ptr()),
        )
    });
    assert!(
        ((((libc::strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>(),
            (c"day %j 100%%".as_ptr().cast_mut()).cast_const(),
            (&mut tm as *mut ::libc::tm).cast_const()
        )) > (0_usize)) as i32)
            != 0)
    );
    (unsafe {
        libc::printf(
            (c"%s\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (buf.as_mut_ptr()),
        )
    });
    assert!(
        ((((libc::strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>(),
            (c"%e".as_ptr().cast_mut()).cast_const(),
            (&mut tm as *mut ::libc::tm).cast_const()
        )) > (0_usize)) as i32)
            != 0)
    );
    (unsafe {
        libc::printf(
            (c"%s\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (buf.as_mut_ptr()),
        )
    });
    let mut small: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    assert!(
        ((((libc::strftime(
            small.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4]>(),
            (c"%Y-%m-%d".as_ptr().cast_mut()).cast_const(),
            (&mut tm as *mut ::libc::tm).cast_const()
        )) == (0_usize)) as i32)
            != 0)
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_time_0() });
    (unsafe { test_gmtime_r_2() });
    (unsafe { test_localtime_r_4() });
    (unsafe { test_strftime_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
