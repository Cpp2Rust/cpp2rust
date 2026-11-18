extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct A {
    pub v: i32,
}
impl A {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { v: 1 };
        this
    }
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new_2() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct B {
    pub v: i32,
}
impl B {
    pub unsafe fn new() -> Self {
        let mut this = Self { v: 2 };
        this
    }
}
impl Default for B {
    fn default() -> Self {
        unsafe { B::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NoDefault {
    pub v: i32,
}
impl NoDefault {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
}
pub unsafe fn used_0(mut x: A) -> i32 {
    return x.v;
}
pub unsafe fn used_1(mut x: B) -> i32 {
    return x.v;
}
pub unsafe fn scaled_2(mut x: A, mut n: i32) -> i32 {
    return ((x.v) * (n));
}
pub unsafe fn always_given_3(mut x: NoDefault) -> i32 {
    return x.v;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct S_NoDefault_ {
    pub v: i32,
}
impl S_NoDefault_ {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn get(&self, mut t: NoDefault) -> i32 {
        return ((self.v) + (t.v));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { used_0(A::new_2(),) }) == (1)));
    assert!(((unsafe { used_0(A::new_1({ 5 },),) }) == (5)));
    assert!(((unsafe { used_1(B::new(),) }) == (2)));
    assert!(
        ((unsafe { scaled_2(A::new_1({ 3 },), (::std::mem::size_of::<A>() as i32),) })
            == ((3) * (::std::mem::size_of::<A>() as i32)))
    );
    assert!(((unsafe { scaled_2(A::new_1({ 3 },), 2,) }) == (6)));
    assert!(((unsafe { always_given_3(NoDefault::new({ 3 },),) }) == (3)));
    let mut s: S_NoDefault_ = S_NoDefault_::new({ 1 });
    assert!(((unsafe { S_NoDefault_::get(&s, NoDefault::new({ 4 },),) }) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
