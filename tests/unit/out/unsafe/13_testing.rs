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
    let mut a: i32 = 1;
    let r: *mut i32 = &mut a as *mut i32;
    let mut p: *mut i32 = (&mut a as *mut i32);
    (*r) = 2;
    (*p) = 3;
    let mut p2: *mut *mut i32 = (&mut p as *mut *mut i32);
    (*(*p2)) = 3;
    return 0;
}
