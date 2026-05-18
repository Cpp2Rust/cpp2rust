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
    let mut val: u32 = 67305985_u32;
    let mut bytes: *mut u8 = ((&mut val as *mut u32) as *mut u8);
    let mut x: u8 = (*bytes.offset((4) as isize));
    &(x);
    return 0;
}
