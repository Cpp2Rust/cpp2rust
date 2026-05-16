extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first: i32 = unsafe { 0_i32 };
pub static mut second: i32 = unsafe { ((first) + (1)) };
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((first) == (0)));
    assert!(((second) == ((first) + (1))));
    return 0;
}
