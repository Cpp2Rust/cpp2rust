extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fp: *mut ::libc::FILE = libc::fopen(
        (c"/tmp/cpp2rust_uafc_test.tmp".as_ptr().cast_mut()).cast_const(),
        (c"wb".as_ptr().cast_mut()).cast_const(),
    );
    assert!(!(fp).is_null());
    libc::fclose(fp);
    return if ((((libc::fputc(('x' as i32), fp)) == ('x' as i32)) as i32) != 0) {
        1
    } else {
        0
    };
}
