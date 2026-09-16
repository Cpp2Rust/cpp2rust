extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_0(mut t: Local) -> i32 {
    return t.x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut l: Local = Local { x: 7 };
    assert!(((unsafe { get_0(l,) }) == (7)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Local {
    pub x: i32,
}
