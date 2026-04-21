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
    let mut x: *mut i32 = (Box::leak(Box::new(5)) as *mut i32);
    let mut out: i32 = (*x);
    ::std::mem::drop(Box::from_raw(x));
    return out;
}
