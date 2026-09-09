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
pub struct A {
    pub a: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct B {
    pub __base: A,
    pub b: i32,
}
impl B {
    pub unsafe fn B(mut x: i32) -> Self {
        let mut this = Self {
            __base: <A>::default(),
            b: ((x) + (1)),
        };
        (this as *mut A).a = x;
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct C {
    pub __base: B,
}
impl C {
    pub unsafe fn sum(&mut self) -> i32 {
        return (((self as *mut A).a) + ((self as *mut B).b));
    }
}
pub unsafe fn geta_0(x: *const A) -> i32 {
    return (*x).a;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: C = C::C1({ 1 });
    assert!(((unsafe { C::sum(&mut c,) }) == (3)));
    assert!(((unsafe { geta_0(&c as *const A,) }) == (1)));
    return 0;
}
