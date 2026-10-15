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
pub struct S_2_ {
    pub v: i32,
    pub w: i32,
}
impl S_2_ {
    pub unsafe fn get(&self) -> i32 {
        return ((self.v) - (self.w));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_1_ {
    pub v: i32,
}
impl S_1_ {
    pub unsafe fn get(&self) -> i32 {
        return ((self.v) * (1));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_3_ {
    pub v: i32,
}
impl S_3_ {
    pub unsafe fn get(&self) -> i32 {
        return ((self.v) * (3));
    }
}
pub unsafe fn scale_0(mut x: i32) -> i32 {
    return ((x) * (1));
}
pub unsafe fn scale_1(mut x: i32) -> i32 {
    return ((x) * (3));
}
pub unsafe fn scale_2(mut x: i32) -> i32 {
    return ((x) - (2));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s1: S_1_ = S_1_ { v: 5 };
    let mut s2: S_2_ = S_2_ { v: 5, w: 1 };
    let mut s3: S_3_ = S_3_ { v: 5 };
    assert!(((unsafe { S_1_::get(&s1,) }) == (5)));
    assert!(((unsafe { S_2_::get(&s2,) }) == (4)));
    assert!(((unsafe { S_3_::get(&s3,) }) == (15)));
    assert!(((unsafe { scale_0(7,) }) == (7)));
    assert!(((unsafe { scale_2(7,) }) == (5)));
    assert!(((unsafe { scale_1(7,) }) == (21)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
