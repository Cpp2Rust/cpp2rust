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
    let mut x: i32 = 5;
    let mut p1: *const i32 = (&mut x as *mut i32).cast_const();
    let mut p2: *const i32 = (&mut x as *mut i32).cast_const();
    if ((p1) != (p2)) {
        return 1;
    } else {
        return -1_i32;
    }
    panic!("ub: non-void function does not return a value")
}
