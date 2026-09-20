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
pub struct A {
    pub v: i32,
}
impl A {
    pub unsafe fn A1() -> Self {
        let mut this = Self { v: 1 };
        this
    }
    pub unsafe fn A2(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::A1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct B {
    pub v: i32,
}
impl B {
    pub unsafe fn B() -> Self {
        let mut this = Self { v: 2 };
        this
    }
}
impl Default for B {
    fn default() -> Self {
        unsafe { B::B() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct NoDefault {
    pub v: i32,
}
impl NoDefault {
    pub unsafe fn NoDefault(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
}
pub unsafe fn used_0(mut x: Option<A>) -> i32 {
    let mut x: A = x.unwrap_or(A::A1());
    return x.v;
}
pub unsafe fn used_1(mut x: Option<B>) -> i32 {
    let mut x: B = x.unwrap_or(B::B());
    return x.v;
}
pub unsafe fn scaled_2(mut x: A, mut n: Option<i32>) -> i32 {
    let mut n: i32 = n.unwrap_or((::std::mem::size_of::<A>() as i32));
    return ((x.v) * (n));
}
pub unsafe fn always_given_3(mut x: Option<NoDefault>) -> i32 {
    let mut x: NoDefault = x.unwrap_or();
    return x.v;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S_NoDefault_ {
    pub v: i32,
}
impl S_NoDefault_ {
    pub unsafe fn S_NoDefault_(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn get(&self, mut t: Option<NoDefault>) -> i32 {
        let mut t: NoDefault = t.unwrap_or();
        return ((self.v) + (t.v));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { used_0(None,) }) == (1)));
    assert!(((unsafe { used_0(Some(A::A2({ 5 },)),) }) == (5)));
    assert!(((unsafe { used_1(None,) }) == (2)));
    assert!(
        ((unsafe { scaled_2(A::A2({ 3 },), None,) })
            == ((3) * (::std::mem::size_of::<A>() as i32)))
    );
    assert!(((unsafe { scaled_2(A::A2({ 3 },), Some(2),) }) == (6)));
    assert!(((unsafe { always_given_3(Some(NoDefault::NoDefault({ 3 },)),) }) == (3)));
    let mut s: S_NoDefault_ = S_NoDefault_::S_NoDefault_({ 1 });
    assert!(((unsafe { S_NoDefault_::get(&s, Some(NoDefault::NoDefault({ 4 },)),) }) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
