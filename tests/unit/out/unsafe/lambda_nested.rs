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
    pub unsafe fn nested_this(&mut self) -> i32 {
        let mut outer: lambda_0 = (lambda_0 {
            this_: (self as *mut S),
        });
        return (unsafe { lambda_0::operator_call(&outer, 20) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    this_: *mut S,
    y: i32,
}
impl lambda_1 {
    pub unsafe fn operator_call(&self, mut z: i32) -> i32 {
        return ((((*self.this_).v) + (self.y)) + (z));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_1::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {
    this_: *mut S,
}
impl lambda_0 {
    pub unsafe fn operator_call(&self, mut y: i32) -> i32 {
        let mut inner: lambda_1 = (lambda_1 {
            this_: self.this_,
            y: y,
        });
        return (unsafe { lambda_1::operator_call(&inner, 1) });
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_0::operator_call(self, a1) }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 10;
    let mut outer: lambda_2 = (lambda_2 { x: &mut x });
    assert!(((unsafe { lambda_2::operator_call(&outer, 20,) }) == (31)));
    x = 100;
    assert!(((unsafe { lambda_2::operator_call(&outer, 20,) }) == (121)));
    let mut s: S = S { v: 5 };
    assert!(((unsafe { S::nested_this(&mut s,) }) == (26)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_3 {
    x: *mut i32,
    y: i32,
}
impl lambda_3 {
    pub unsafe fn operator_call(&self, mut z: i32) -> i32 {
        return (((*self.x) + (self.y)) + (z));
    }
}
impl Callable1<i32, i32> for lambda_3 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_3::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {
    x: *mut i32,
}
impl lambda_2 {
    pub unsafe fn operator_call(&self, mut y: i32) -> i32 {
        let mut inner: lambda_3 = (lambda_3 { x: self.x, y: y });
        return (unsafe { lambda_3::operator_call(&inner, 1) });
    }
}
impl Callable1<i32, i32> for lambda_2 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_2::operator_call(self, a1) }
    }
}
