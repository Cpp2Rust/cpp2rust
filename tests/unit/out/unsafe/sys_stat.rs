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
    assert!(((((st.st_size) == (5 as ::libc::off_t)) as i32) != 0));
    assert!((((((*(&mut st as *mut ::libc::stat)).st_mtime) > (0 as ::libc::time_t)) as i32) != 0));
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
    assert!(((((st.st_size) == (11 as ::libc::off_t)) as i32) != 0));
    assert!((((((*(&mut st as *mut ::libc::stat)).st_mtime) > (0 as ::libc::time_t)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    libc::unlink(path);
}
pub unsafe fn test_st_mtime_access_2() {
    let mut st: ::libc::stat = unsafe { std::mem::zeroed() };
    let mut sp: *mut ::libc::stat = (&mut st as *mut ::libc::stat);
    (*(&mut st as *mut ::libc::stat)).st_mtime = (5 as ::libc::time_t);
    assert!(
        (((((*(&mut st as *mut ::libc::stat)).st_mtime) == (5 as ::libc::time_t)) as i32) != 0)
    );
    (*(sp as *mut ::libc::stat)).st_mtime = (6 as ::libc::time_t);
    assert!((((((*(sp as *mut ::libc::stat)).st_mtime) == (6 as ::libc::time_t)) as i32) != 0));
    (*(&mut st as *mut ::libc::stat)).st_mtime += (2 as ::libc::time_t);
    assert!(
        (((((*(&mut st as *mut ::libc::stat)).st_mtime) == (8 as ::libc::time_t)) as i32) != 0)
    );
    (*(sp as *mut ::libc::stat)).st_mtime -= (1 as ::libc::time_t);
    assert!((((((*(sp as *mut ::libc::stat)).st_mtime) == (7 as ::libc::time_t)) as i32) != 0));
    (*(&mut st as *mut ::libc::stat)).st_mtime.postfix_inc();
    (*(sp as *mut ::libc::stat)).st_mtime.prefix_inc();
    assert!(
        (((((*(&mut st as *mut ::libc::stat)).st_mtime) == (9 as ::libc::time_t)) as i32) != 0)
    );
    let mut mtime: *mut ::libc::time_t =
        (&mut (*(&mut st as *mut ::libc::stat)).st_mtime as *mut ::libc::time_t);
    (*mtime) = (10 as ::libc::time_t);
    assert!(
        (((((*(&mut st as *mut ::libc::stat)).st_mtime) == (10 as ::libc::time_t)) as i32) != 0)
    );
    let mut mtime_via_ptr: *mut ::libc::time_t =
        (&mut (*(sp as *mut ::libc::stat)).st_mtime as *mut ::libc::time_t);
    (*mtime_via_ptr) = (11 as ::libc::time_t);
    assert!(((((*mtime) == (11 as ::libc::time_t)) as i32) != 0));
    let mut copy: ::libc::time_t = (*(sp as *mut ::libc::stat)).st_mtime;
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
