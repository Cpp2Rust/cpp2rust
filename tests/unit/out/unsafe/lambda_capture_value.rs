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
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut factor: i32 = 3;
    let mut scale: lambda_0 = (lambda_0 { factor: factor });
    assert!(((unsafe { lambda_0::operator_call(&scale, 4,) }) == (12)));
    factor = 100;
    assert!(((unsafe { lambda_0::operator_call(&scale, 4,) }) == (12)));
    let mut slot: i32 = 7;
    let mut p: *mut i32 = (&mut slot as *mut i32);
    let mut read_ptr: lambda_1 = (lambda_1 { p: p });
    slot = 8;
    assert!(((unsafe { lambda_1::operator_call(&read_ptr,) }) == (8)));
    let mut s: S = S { x: 1, y: 2 };
    let mut sum: lambda_2 = (lambda_2 { s: s });
    s.x = 50;
    assert!(((unsafe { lambda_2::operator_call(&sum,) }) == (3)));
    let mut base: i32 = 10;
    let mut shifted: lambda_3 = (lambda_3 { y: ((base) + (1)) });
    assert!(((unsafe { lambda_3::operator_call(&shifted, 5,) }) == (16)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {
    factor: i32,
}
impl lambda_0 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return ((x) * (self.factor));
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_0::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    p: *mut i32,
}
impl lambda_1 {
    pub unsafe fn operator_call(&self) -> i32 {
        return (*self.p);
    }
}
impl Callable0<i32> for lambda_1 {
    fn call(&self) -> i32 {
        unsafe { lambda_1::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {
    s: S,
}
impl lambda_2 {
    pub unsafe fn operator_call(&self) -> i32 {
        return ((self.s.x) + (self.s.y));
    }
}
impl Callable0<i32> for lambda_2 {
    fn call(&self) -> i32 {
        unsafe { lambda_2::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_3 {
    y: i32,
}
impl lambda_3 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return ((x) + (self.y));
    }
}
impl Callable1<i32, i32> for lambda_3 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_3::operator_call(self, a1) }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
