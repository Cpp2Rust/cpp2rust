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
    let mut h: i32 = 15;
    let h_ref1: *mut i32 = &mut h as *mut i32;
    (*h_ref1) = 16;
    let mut h_ptr: *mut i32 = (h_ref1);
    let h_ref2: *mut i32 = &mut (*h_ptr) as *mut i32;
    (*h_ref2) = 17;
    return ((*h_ref1) + (*h_ref2));
}
