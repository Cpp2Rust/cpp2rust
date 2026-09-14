extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn a_0() -> i32 {
    static mut i_1: i32 = unsafe { 1 };;
    return (*&raw mut i_1);
}
pub unsafe fn b_2() -> i32 {
    static mut i_3: i32 = unsafe { 2 };;
    return (*&raw mut i_3);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { a_0() }) == (1)));
    assert!(((unsafe { b_2() }) == (2)));
    return 0;
}
