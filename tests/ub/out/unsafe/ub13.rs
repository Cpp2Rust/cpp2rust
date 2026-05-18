extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn escape_0(mut p: *mut i32) {
    ::std::mem::drop(Box::from_raw(p));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p1: *mut i32 = (Box::leak(Box::new(1)) as *mut i32);
    (unsafe {
        let _p: *mut i32 = p1;
        escape_0(_p)
    });
    return (*p1);
}
