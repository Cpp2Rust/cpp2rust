extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_stat_0() {
    let mut path: *const ::core::ffi::c_char =
        (c"/tmp/cpp2rust_stat_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello".as_ptr().cast_mut()).cast_const(), fp);
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    let mut st: stat = unsafe { std::mem::zeroed::<stat>() };
    assert!(((((libc::stat(path, (&mut st as *mut stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (5_i64)) as i32) != 0));
    assert!(((((st.st_mtime) > (0_i64)) as i32) != 0));
    libc::unlink(path);
}
pub unsafe fn test_fstat_1() {
    let mut path: *const ::core::ffi::c_char =
        (c"/tmp/cpp2rust_fstat_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello world".as_ptr().cast_mut()).cast_const(), fp);
    libc::fflush(fp);
    let mut fd: i32 = libc::fileno(fp);
    let mut st: stat = unsafe { std::mem::zeroed::<stat>() };
    assert!(((((libc::fstat(fd, (&mut st as *mut stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (11_i64)) as i32) != 0));
    assert!(((((st.st_mtime) > (0_i64)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    libc::unlink(path);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_stat_0() });
    (unsafe { test_fstat_1() });
    return 0;
}
