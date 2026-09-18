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
    pub n: i32,
    pub step: i32,
}
impl S {
    pub unsafe fn add(&mut self, mut k: i32) {
        self.n += k;
    }
    pub unsafe fn scaled(&self) -> i32 {
        return ((self.n) * (self.step));
    }
    pub unsafe fn bump(&mut self, mut by: i32) {
        let mut inc: lambda_0 = (lambda_0 {
            this_: (self as *mut S),
        });
        (unsafe { lambda_0::operator_call(&inc, by) });
        (unsafe { lambda_0::operator_call(&inc, by) });
    }
    pub unsafe fn bump_via_method(&mut self, mut by: i32) {
        let mut inc: lambda_1 = (lambda_1 {
            this_: (self as *mut S),
        });
        (unsafe { lambda_1::operator_call(&inc, by) });
    }
    pub unsafe fn read_scaled(&self) -> i32 {
        let mut get: lambda_2 = (lambda_2 {
            this_: (self as *const S),
        });
        return (unsafe { lambda_2::operator_call(&get) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {
    this_: *mut S,
}
impl lambda_0 {
    pub unsafe fn operator_call(&self, mut k: i32) {
        (*self.this_).n += k;
    }
}
impl Callable1<i32, ()> for lambda_0 {
    fn call(&self, a1: i32) -> () {
        unsafe { lambda_0::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    this_: *mut S,
}
impl lambda_1 {
    pub unsafe fn operator_call(&self, mut k: i32) {
        (unsafe { S::add(&mut (*self.this_), k) });
    }
}
impl Callable1<i32, ()> for lambda_1 {
    fn call(&self, a1: i32) -> () {
        unsafe { lambda_1::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {
    this_: *const S,
}
impl lambda_2 {
    pub unsafe fn operator_call(&self) -> i32 {
        return (unsafe { S::scaled(&(*self.this_)) });
    }
}
impl Callable0<i32> for lambda_2 {
    fn call(&self) -> i32 {
        unsafe { lambda_2::operator_call(self) }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { n: 0, step: 2 };
    (unsafe { S::bump(&mut s, 3) });
    assert!(((s.n) == (6)));
    (unsafe { S::bump_via_method(&mut s, 4) });
    assert!(((s.n) == (10)));
    assert!(((unsafe { S::read_scaled(&s,) }) == (20)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
