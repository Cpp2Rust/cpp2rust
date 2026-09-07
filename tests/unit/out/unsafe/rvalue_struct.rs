extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub a: i32,
    pub b: i32,
}
impl S {
    pub unsafe fn S(mut a: i32, mut b: i32) -> Self {
        let mut this = Self { a: a, b: b };
        this
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s1: S = S::S({ 1 }, { 2 });
    let s2: *mut S = &mut s1;
    assert!((((*s2).a) == (1)));
    assert!((((*s2).b) == (2)));
    return 0;
}
