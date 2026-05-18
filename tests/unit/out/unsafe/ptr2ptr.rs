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
    let mut out: i32 = 0;
    let mut x: i32 = 0;
    let mut p1: *mut i32 = (&mut x as *mut i32);
    (*p1) = 1;
    out *= x;
    let mut p2: *mut *mut i32 = (&mut p1 as *mut *mut i32);
    (*(*p2)) = 2;
    out *= x;
    let mut p3: *mut *mut *mut i32 = (&mut p2 as *mut *mut *mut i32);
    (*(*(*p3))) = 3;
    out *= x;
    return out;
}
