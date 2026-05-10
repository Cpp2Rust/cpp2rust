extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn null_0(mut p: *mut *mut i32) {
    (*p) = std::ptr::null_mut();
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 1;
    let mut p: *mut i32 = (&mut x as *mut i32);
    (unsafe {
        let _p: *mut *mut i32 = (&mut p as *mut *mut i32);
        null_0(_p)
    });
    let r: *mut i32 = &mut (*p) as *mut i32;
    return (*r);
}
