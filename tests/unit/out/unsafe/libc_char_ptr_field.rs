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
    let mut pw: *mut passwd = libc::getpwuid(libc::geteuid());
    if !!(pw).is_null() {
        return 0;
    }
    let mut home: *mut u8 = ((*pw).pw_dir as *mut u8);
    return (((home).is_null()) as i32);
}
