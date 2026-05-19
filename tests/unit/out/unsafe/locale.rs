extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_setlocale_0() {
    let mut cur: *mut u8 = libc::setlocale(6, std::ptr::null() as *const i8) as *mut u8;
    assert!((((!((cur).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            (cur).cast_const() as *const i8,
            (b"C\0".as_ptr().cast_mut()).cast_const() as *const i8
        )) == (0)) as i32)
            != 0)
    );
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_setlocale_0() });
    return 0;
}
