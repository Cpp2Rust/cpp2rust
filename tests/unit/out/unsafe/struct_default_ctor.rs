extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub a: i32,
    pub b: bool,
}
impl S {
    pub unsafe fn S() -> Self {
        let mut this = Self { a: 11, b: true };
        this
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::S() }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::S();
    assert!(((s.a) == (11)));
    assert!(((s.b as i32) == (true as i32)));
    return 0;
}
