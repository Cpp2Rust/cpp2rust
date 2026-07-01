extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fopen_0(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut ::libc::FILE {
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
        fopen_0(
            (c"/tmp/irrelevant-file".as_ptr().cast_mut()).cast_const(),
            (c"r".as_ptr().cast_mut()).cast_const(),
        )
    });
    assert!(((((fp).is_null()) as i32) != 0));
    return 0;
}
