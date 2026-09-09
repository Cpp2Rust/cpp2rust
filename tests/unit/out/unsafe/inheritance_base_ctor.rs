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
pub struct Base {
    pub a_: i16,
    pub b_: i8,
}
impl Base {
    pub unsafe fn Base(mut a: i16, mut b: i8) -> Self {
        let mut this = Self { a_: a, b_: b };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Derived {
    pub __base: Base,
    pub c_: i8,
}
impl Derived {
    pub unsafe fn Derived(mut a: i16, mut b: i8, mut c: i8) -> Self {
        let mut this = Self {
            __base: Base::Base({ a }, { b }),
            c_: c,
        };
        this
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut src: Derived = Derived::Derived({ 1_i16 }, { 2_i8 }, { 3_i8 });
    let mut dst: Derived = Derived::Derived({ 4_i16 }, { 5_i8 }, { 6_i8 });
    let mut s: *mut Base = (&mut src as *mut Derived);
    let mut t: *mut Base = (&mut dst as *mut Derived);
    (*t) = (*s).clone();
    assert!((((dst as Base).a_ as i32) == (1)));
    assert!((((dst as Base).b_ as i32) == (2)));
    assert!(((dst.c_ as i32) == (6)));
    return 0;
}
