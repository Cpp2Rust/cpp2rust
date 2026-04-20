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
    let mut a0: f64 = 3.14E+0;
    let mut a1: f64 = 2.71E+0;
    if *&mut a0 <= *&mut a1 {
        (&mut a0) as *const _
    } else {
        (&mut a1) as *const _
    };
    return 0;
}
