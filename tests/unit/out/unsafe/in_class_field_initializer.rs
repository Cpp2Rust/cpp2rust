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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Boxed_int_ {
    pub v: i32,
    pub tag: i32,
}
impl Boxed_int_ {
    pub unsafe fn Boxed_int_(mut x: i32, mut t: i32) -> Self {
        let mut this = Self { v: x, tag: t };
        this
    }
}
impl Default for Boxed_int_ {
    fn default() -> Self {
        Boxed_int_ {
            v: 0_i32,
            tag: 0_i32,
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
    let mut boxed: Boxed_int_ = Boxed_int_::Boxed_int_({ 5 }, { 9 });
    assert!(((boxed.v) == (5)));
    assert!(((boxed.tag) == (9)));
    return 0;
}
