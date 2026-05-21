extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fopen_0(mut path: *const u8, mut mode: *const u8) -> *mut ::libc::FILE {
    &(path);
    &(mode);
    return std::ptr::null_mut();
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fp: *mut ::libc::FILE = (unsafe {
        let _path: *const u8 = (b"/tmp/irrelevant-file\0".as_ptr().cast_mut()).cast_const();
        let _mode: *const u8 = (b"r\0".as_ptr().cast_mut()).cast_const();
        fopen_0(_path, _mode)
    });
    assert!(((((fp).is_null()) as i32) != 0));
    return 0;
}
