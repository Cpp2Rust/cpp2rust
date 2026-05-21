extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_stat_0() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_stat_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(
        path as *const i8,
        (b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs(
        (b"hello\0".as_ptr().cast_mut()).cast_const() as *const i8,
        fp,
    );
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    let mut st: stat = std::mem::zeroed::<stat>();
    assert!(((((libc::stat(path as *const i8, (&mut st as *mut stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (5_i64)) as i32) != 0));
    libc::unlink(path as *const i8);
}
pub unsafe fn test_fstat_1() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_fstat_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(
        path as *const i8,
        (b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8,
    );
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs(
        (b"hello world\0".as_ptr().cast_mut()).cast_const() as *const i8,
        fp,
    );
    libc::fflush(fp);
    let mut fd: i32 = libc::fileno(fp);
    let mut st: stat = std::mem::zeroed::<stat>();
    assert!(((((libc::fstat(fd, (&mut st as *mut stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (11_i64)) as i32) != 0));
    assert!(((((libc::fclose(fp)) == (0)) as i32) != 0));
    libc::unlink(path as *const i8);
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
