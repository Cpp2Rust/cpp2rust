extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn dangling_0() -> *mut i32 {
    let mut x: i32 = 1;
    let mut p: *mut i32 = (&mut x as *mut i32);
    return &mut (*p) as *mut i32;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let x: *mut i32 = (unsafe { dangling_0() });
    return (*x);
}
