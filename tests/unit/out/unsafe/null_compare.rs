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
    let mut a: i32 = 0;
    let mut p: *mut i32 = (&mut a as *mut i32);
    assert!((((!((p).is_null())) as i32) != 0));
    assert!((((!((p).is_null())) as i32) != 0));
    p = std::ptr::null_mut();
    assert!(((((p).is_null()) as i32) != 0));
    assert!(((((p).is_null()) as i32) != 0));
    return 0;
}
