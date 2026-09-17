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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 3;
    let mut by_value: lambda_0 = (lambda_0 { a: a, b: b, c: c });
    assert!(((unsafe { lambda_0::operator_call(&by_value, 10,) }) == (16)));
    a = 100;
    assert!(((unsafe { lambda_0::operator_call(&by_value, 10,) }) == (16)));
    let mut by_ref: lambda_1 = (lambda_1 {
        a: &mut a,
        b: &mut b,
        c: &mut c,
    });
    assert!(((unsafe { lambda_1::operator_call(&by_ref, 10,) }) == (115)));
    b = 200;
    assert!(((unsafe { lambda_1::operator_call(&by_ref, 10,) }) == (313)));
    let mut mixed: lambda_2 = (lambda_2 {
        c: &mut c,
        a: a,
        b: b,
    });
    assert!(((unsafe { lambda_2::operator_call(&mixed, 1,) }) == (((100) + (200)) + (4))));
    assert!(((c) == (4)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {
    a: i32,
    b: i32,
    c: i32,
}
impl lambda_0 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return ((((self.a) + (self.b)) + (self.c)) + (x));
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
    a: *mut i32,
    b: *mut i32,
    c: *mut i32,
}
impl lambda_1 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return ((((*self.a) + (*self.b)) + (*self.c)) + (x));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_1::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {
    c: *mut i32,
    a: i32,
    b: i32,
}
impl lambda_2 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        (*self.c) += x;
        return (((self.a) + (self.b)) + (*self.c));
    }
}
impl Callable1<i32, i32> for lambda_2 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_2::operator_call(self, a1) }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
