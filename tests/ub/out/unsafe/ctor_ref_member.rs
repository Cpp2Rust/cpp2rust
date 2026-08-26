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
    pub r: *const i32,
}
impl S {
    pub unsafe fn S(x: *const i32) -> Self {
        let mut this = Self { r: x };
        this
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::S({
        let mut __tmp_0: i32 = 5;
        &mut __tmp_0
    });
    assert!(((*s.r) == (5)));
    return 0;
}
