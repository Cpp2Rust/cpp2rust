extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 10;
    let mut outer: lambda_0 = (lambda_0 { x: &mut x });
    assert!(((unsafe { lambda_0::operator_call(&outer, 20,) }) == (31)));
    x = 100;
    assert!(((unsafe { lambda_0::operator_call(&outer, 20,) }) == (121)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    x: *mut i32,
    y: i32,
}
impl lambda_1 {
    pub unsafe fn operator_call(&self, mut z: i32) -> i32 {
        return (((*self.x) + (self.y)) + (z));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        let __this: lambda_1 = self.clone();
        unsafe { lambda_1::operator_call(&__this, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {
    x: *mut i32,
}
impl lambda_0 {
    pub unsafe fn operator_call(&self, mut y: i32) -> i32 {
        let mut inner: lambda_1 = (lambda_1 {
            x: &mut (*self.x),
            y: y,
        });
        return (unsafe { lambda_1::operator_call(&inner, 1) });
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        let __this: lambda_0 = self.clone();
        unsafe { lambda_0::operator_call(&__this, a1) }
    }
}
