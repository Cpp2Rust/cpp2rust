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
pub struct Inner {
    pub x: i32,
    pub y: i32,
}
impl Default for Inner {
    fn default() -> Self {
        Inner { x: 3, y: 4 }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub a: i32,
    pub b: libc::c_char,
    pub c: Inner,
    pub d: Inner,
}
impl Default for S {
    fn default() -> Self {
        S {
            a: 1,
            b: (2 as libc::c_char),
            c: Inner { x: 3, y: 4 },
            d: <Inner>::default(),
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = <S>::default();
    assert!(((s.a) == (1)));
    assert!(((s.b as i32) == (2)));
    assert!(((s.c.x) == (3)));
    assert!(((s.c.y) == (4)));
    assert!(((s.d.x) == (3)));
    assert!(((s.d.y) == (4)));
    return 0;
}
