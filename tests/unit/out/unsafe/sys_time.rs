extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_time_0() {
    let mut t1: i64 = libc::time(std::ptr::null_mut());
    let mut t2: i64 = 0_i64;
    let mut t3: i64 = libc::time((&mut t2 as *mut i64));
    assert!(((((t1) > (1500000000_i64)) as i32) != 0));
    assert!(((((t2) == (t3)) as i32) != 0));
    assert!(((((t3) >= (t1)) as i32) != 0));
}
pub unsafe fn test_clock_gettime_1() {
    let mut ts: ::libc::timespec = unsafe { std::mem::zeroed() };
    assert!(((((libc::clock_gettime(0, (&mut ts as *mut ::libc::timespec))) == (0)) as i32) != 0));
    assert!(((((ts.tv_sec) > (1500000000_i64)) as i32) != 0));
    assert!(
        (((((((ts.tv_nsec) >= (0_i64)) as i32) != 0)
            && ((((ts.tv_nsec) < (1000000000_i64)) as i32) != 0)) as i32)
            != 0)
    );
}
pub unsafe fn print_tm_2(mut t: i64) {
    let mut tm: ::libc::tm = unsafe { std::mem::zeroed() };
    assert!(
        (((!((libc::gmtime_r(
            (&mut t as *mut i64).cast_const(),
            (&mut tm as *mut ::libc::tm)
        ))
        .is_null())) as i32)
            != 0)
    );
    printf(
        (c"%d-%d-%d %d:%d:%d wday=%d yday=%d %s gmtoff=%ld isdst=%d\n"
            .as_ptr()
            .cast_mut())
        .cast_const() as *const i8,
        tm.tm_year,
        tm.tm_mon,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        tm.tm_wday,
        tm.tm_yday,
        tm.tm_zone,
        tm.tm_gmtoff,
        tm.tm_isdst,
    );
}
pub unsafe fn test_gmtime_r_3() {
    (unsafe { print_tm_2(0_i64) });
    (unsafe { print_tm_2(1_i64) });
    (unsafe { print_tm_2(86399_i64) });
    (unsafe { print_tm_2(86400_i64) });
    (unsafe { print_tm_2(951782400_i64) });
    (unsafe { print_tm_2(951868799_i64) });
    (unsafe { print_tm_2(1704067199_i64) });
    (unsafe { print_tm_2(1704067200_i64) });
    (unsafe { print_tm_2(1721126096_i64) });
    (unsafe { print_tm_2(4102444800_i64) });
}
pub unsafe fn test_strftime_4() {
    let mut t: i64 = 1721126096_i64;
    let mut tm: ::libc::tm = unsafe { std::mem::zeroed() };
    assert!(
        (((!((libc::gmtime_r(
            (&mut t as *mut i64).cast_const(),
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
    printf(
        (c"%s\n".as_ptr().cast_mut()).cast_const() as *const i8,
        buf.as_mut_ptr(),
    );
    assert!(
        ((((libc::strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>(),
            (c"%a, %d %b %Y %T".as_ptr().cast_mut()).cast_const(),
            (&mut tm as *mut ::libc::tm).cast_const()
        )) > (0_usize)) as i32)
            != 0)
    );
    printf(
        (c"%s\n".as_ptr().cast_mut()).cast_const() as *const i8,
        buf.as_mut_ptr(),
    );
    assert!(
        ((((libc::strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>(),
            (c"day %j 100%%".as_ptr().cast_mut()).cast_const(),
            (&mut tm as *mut ::libc::tm).cast_const()
        )) > (0_usize)) as i32)
            != 0)
    );
    printf(
        (c"%s\n".as_ptr().cast_mut()).cast_const() as *const i8,
        buf.as_mut_ptr(),
    );
    assert!(
        ((((libc::strftime(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>(),
            (c"%e".as_ptr().cast_mut()).cast_const(),
            (&mut tm as *mut ::libc::tm).cast_const()
        )) > (0_usize)) as i32)
            != 0)
    );
    printf(
        (c"%s\n".as_ptr().cast_mut()).cast_const() as *const i8,
        buf.as_mut_ptr(),
    );
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
pub unsafe fn test_gettimeofday_5() {
    let mut tv: ::libc::timeval = unsafe { std::mem::zeroed() };
    assert!(
        ((((libc::gettimeofday(
            (&mut tv as *mut ::libc::timeval),
            (0 as *mut ::libc::c_void) as *mut libc::timezone
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((tv.tv_sec) > (1500000000_i64)) as i32) != 0));
    assert!(
        (((((((tv.tv_usec) >= (0_i64)) as i32) != 0)
            && ((((tv.tv_usec) < (1000000_i64)) as i32) != 0)) as i32)
            != 0)
    );
}
pub unsafe fn test_utimes_6() {
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_utimes_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    let mut times: [::libc::timeval; 2] = [unsafe { std::mem::zeroed() }; 2];
    times[(0) as usize].tv_sec = 1000000000_i64;
    times[(0) as usize].tv_usec = 0_i64;
    times[(1) as usize].tv_sec = 1000000001_i64;
    times[(1) as usize].tv_usec = 0_i64;
    assert!(((((libc::utimes(path, (times.as_mut_ptr()).cast_const())) == (0)) as i32) != 0));
    assert!(
        ((((libc::utimes(
            (c"/tmp/cpp2rust_utimes_test_missing.tmp".as_ptr().cast_mut()).cast_const(),
            (times.as_mut_ptr()).cast_const()
        )) == (-1_i32)) as i32)
            != 0)
    );
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_time_0() });
    (unsafe { test_clock_gettime_1() });
    (unsafe { test_gmtime_r_3() });
    (unsafe { test_strftime_4() });
    (unsafe { test_gettimeofday_5() });
    (unsafe { test_utimes_6() });
    return 0;
}
