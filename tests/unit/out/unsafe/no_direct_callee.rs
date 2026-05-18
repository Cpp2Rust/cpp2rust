extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0() -> bool {
    return false;
}
pub unsafe fn test_1(mut fn_: Option<unsafe fn() -> bool>) -> i32 {
    if !(unsafe { (fn_).unwrap()() }) {
        return 1;
    }
    return 0;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return (unsafe {
        let _fn: Option<unsafe fn() -> bool> = Some(test1_0);
        test_1(_fn)
    });
}
