extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_getpwuid_0() {
    let mut uid: u32 = libc::geteuid();
    let mut pw: *mut passwd = libc::getpwuid(uid);
    assert!((((!((pw).is_null())) as i32) != 0));
    assert!((((((*pw).pw_uid) == (uid)) as i32) != 0));
    printf(
        (b"%s\n\0".as_ptr().cast_mut()).cast_const() as *const i8,
        (*pw).pw_name,
    );
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_getpwuid_0() });
    return 0;
}
