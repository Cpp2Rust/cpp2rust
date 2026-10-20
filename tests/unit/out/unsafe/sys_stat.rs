extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_stat_0() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_stat_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello".as_ptr().cast_mut()).cast_const(), fp);
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    let mut st: ::libc::stat = unsafe { std::mem::zeroed() };
    assert!(((((libc::stat(path, (&mut st as *mut ::libc::stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (5_i64)) as i32) != 0));
    assert!(((((*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime) > (0_i64)) as i32) != 0));
    libc::unlink(path);
}
pub unsafe fn test_fstat_1() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_fstat_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello world".as_ptr().cast_mut()).cast_const(), fp);
    libc::fflush(fp);
    let mut fd: i32 = libc::fileno(fp);
    let mut st: ::libc::stat = unsafe { std::mem::zeroed() };
    assert!(((((libc::fstat(fd, (&mut st as *mut ::libc::stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (11_i64)) as i32) != 0));
    assert!(((((*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime) > (0_i64)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    libc::unlink(path);
}
pub unsafe fn test_st_mtime_access_2() {
    let mut st: ::libc::stat = unsafe { std::mem::zeroed() };
    let mut sp: *mut ::libc::stat = (&mut st as *mut ::libc::stat);
    (*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime) = 5_i64;
    assert!(((((*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime) == (5_i64)) as i32) != 0));
    (*&raw mut (*(sp as *mut ::libc::stat)).st_mtime) = 6_i64;
    assert!(((((*&raw mut (*(sp as *mut ::libc::stat)).st_mtime) == (6_i64)) as i32) != 0));
    (*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime) += 2_i64;
    assert!(((((*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime) == (8_i64)) as i32) != 0));
    (*&raw mut (*(sp as *mut ::libc::stat)).st_mtime) -= 1_i64;
    assert!(((((*&raw mut (*(sp as *mut ::libc::stat)).st_mtime) == (7_i64)) as i32) != 0));
    (*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime).postfix_inc();
    (*&raw mut (*(sp as *mut ::libc::stat)).st_mtime).prefix_inc();
    assert!(((((*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime) == (9_i64)) as i32) != 0));
    let mut mtime: *mut ::libc::time_t = ((&mut (*&raw mut (*(&mut st as *mut ::libc::stat))
        .st_mtime) as *mut i64) as *mut i64)
        .cast::<::libc::time_t>();
    (*mtime) = (10 as ::libc::time_t);
    assert!(((((*&raw mut (*(&mut st as *mut ::libc::stat)).st_mtime) == (10_i64)) as i32) != 0));
    let mut mtime_via_ptr: *mut ::libc::time_t = ((&mut (*&raw mut (*(sp as *mut ::libc::stat))
        .st_mtime) as *mut i64) as *mut i64)
        .cast::<::libc::time_t>();
    (*mtime_via_ptr) = (11 as ::libc::time_t);
    assert!(((((*mtime) == (11 as ::libc::time_t)) as i32) != 0));
    let mut copy: ::libc::time_t =
        ((*&raw mut (*(sp as *mut ::libc::stat)).st_mtime) as ::libc::time_t);
    assert!(((((copy) == (11 as ::libc::time_t)) as i32) != 0));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_stat_0() });
    (unsafe { test_fstat_1() });
    (unsafe { test_st_mtime_access_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
