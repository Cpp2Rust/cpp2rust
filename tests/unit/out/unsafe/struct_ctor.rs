extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StructWithCtor {
    x1_: i32,
    x2_: i32,
}
impl StructWithCtor {
    pub unsafe fn new(mut x1: i32, mut x2: i32) -> Self {
        let mut this = Self { x1_: x1, x2_: x2 };
        this.x1_.prefix_inc();
        this.x2_.prefix_dec();
        this
    }
    pub unsafe fn x1(&self) -> *const i32 {
        return &self.x1_;
    }
    pub unsafe fn x2(&self) -> *const i32 {
        return &self.x2_;
    }
}
pub unsafe fn foo_0(x: *mut i32) -> *mut i32 {
    return x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Value_ {
    pub v: i32,
}
impl Value_ {
    pub unsafe fn new(mut u: i32) -> Self {
        let mut this = Self { v: u };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Ptr_ {
    pub v1: Value_,
    pub v2: Value_,
}
impl Ptr_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            v1: Value_::new({ 11 }),
            v2: Value_::new({ 22 }),
        };
        this
    }
}
impl Default for Ptr_ {
    fn default() -> Self {
        unsafe { Ptr_::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut struct_with_ctor: StructWithCtor = StructWithCtor::new({ 1 }, { 2 });
    let mut x: i32 = 3;
    assert!(
        (((*(unsafe { foo_0(&mut x,) })) == (3))
            && ((*(unsafe { StructWithCtor::x1(&struct_with_ctor,) })) == (2)))
            && ((*(unsafe { StructWithCtor::x2(&struct_with_ctor,) })) == (1))
    );
    let mut p: Ptr_ = Ptr_::new();
    assert!(((p.v1.v) == (11)));
    assert!(((p.v2.v) == (22)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
