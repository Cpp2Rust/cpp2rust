extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn bump_0(mut arg: *mut ::libc::c_void) -> i32 {
    let mut value: *mut i32 = (arg as *mut i32);
    (*value) += 1;
    return (*value);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut value: i32 = 41;
    let mut opaque: *mut ::libc::c_void =
        ((&mut value as *mut i32) as *mut i32 as *mut ::libc::c_void);
    let mut typed: *mut i32 = (opaque as *mut i32);
    assert!(((((unsafe { bump_0(opaque,) }) == (42)) as i32) != 0));
    assert!(((((*typed) == (42)) as i32) != 0));
    (*typed) = 7;
    assert!(((((value) == (7)) as i32) != 0));
    return 0;
}
