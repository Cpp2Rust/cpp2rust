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
    pub v: i32,
}
impl S {
    pub unsafe fn get(&self) -> i32 {
        return self.v;
    }
}
pub unsafe trait Base {
    unsafe fn scale(&mut self, x: i32) -> i32;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct D {
    pub f: i32,
}
impl D {}
unsafe impl Base for D {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::S({ 1 });
    assert!(((unsafe { S::get(&s,) }) == (1)));
    (unsafe { S::set(&mut s, 4) });
    assert!(((unsafe { S::get(&s,) }) == (4)));
    assert!(((unsafe { S::add(&mut s, 2,) }) == (6)));
    let mut d: D = D::D({ 3 });
    let mut b: *mut dyn Base = (&mut d as *mut D);
    assert!(((unsafe { (*b).scale(5,) }) == (15)));
    return 0;
}
impl S {
    pub unsafe fn S(mut x: i32) -> Self {
        let mut this = Self { v: x };
        this
    }
}
impl D {
    pub unsafe fn D(mut f: i32) -> Self {
        let mut this = Self { f: f };
        this
    }
}
impl S {}
impl S {
    pub unsafe fn destructor(&mut self) {}
}
impl S {
    pub unsafe fn set(&mut self, mut x: i32) {
        self.v = x;
    }
}
impl S {
    pub unsafe fn add(&mut self, mut x: i32) -> i32 {
        self.v += x;
        return self.v;
    }
}
impl D {}
pub unsafe fn __cpp2rust_init_globals() {}
