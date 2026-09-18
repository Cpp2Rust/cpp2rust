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
    let mut zero: lambda_0 = (lambda_0 {});
    assert!(((unsafe { lambda_0::operator_call() }) == (42)));
    let mut one: lambda_1 = (lambda_1 {});
    assert!(((unsafe { lambda_1::operator_call(1,) }) == (2)));
    let mut three: lambda_2 = (lambda_2 {});
    assert!(((unsafe { lambda_2::operator_call(1, 2, 3,) }) == (123)));
    let mut hits: i32 = 0;
    let mut no_return: lambda_3 = (lambda_3 { hits: &mut hits });
    (unsafe { lambda_3::operator_call(&no_return, 3) });
    (unsafe { lambda_3::operator_call(&no_return, 4) });
    assert!(((hits) == (7)));
    let mut a: i32 = 2;
    let mut b: i32 = 3;
    let mut product: i32 = (unsafe {
        lambda_4::operator_call(
            &(lambda_4 {
                a: &mut a,
                b: &mut b,
            }),
        )
    });
    assert!(((product) == (6)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {}
impl lambda_0 {
    pub unsafe fn operator_call() -> i32 {
        return 42;
    }
}
impl Callable0<i32> for lambda_0 {
    fn call(&self) -> i32 {
        unsafe { lambda_0::operator_call() }
    }
}
impl lambda_0 {
    pub fn to_free_function(&self) -> Option<unsafe fn() -> i32> {
        Some(lambda_0::operator_call)
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {}
impl lambda_1 {
    pub unsafe fn operator_call(mut x: i32) -> i32 {
        return ((x) + (1));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_1::operator_call(a1) }
    }
}
impl lambda_1 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_1::operator_call)
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {}
impl lambda_2 {
    pub unsafe fn operator_call(mut x: i32, mut y: i32, mut z: i32) -> i32 {
        return ((((x) * (100)) + ((y) * (10))) + (z));
    }
}
impl Callable3<i32, i32, i32, i32> for lambda_2 {
    fn call(&self, a1: i32, a2: i32, a3: i32) -> i32 {
        unsafe { lambda_2::operator_call(a1, a2, a3) }
    }
}
impl lambda_2 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32, i32, i32) -> i32> {
        Some(lambda_2::operator_call)
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_3 {
    hits: *mut i32,
}
impl lambda_3 {
    pub unsafe fn operator_call(&self, mut by: i32) {
        (*self.hits) += by;
    }
}
impl Callable1<i32, ()> for lambda_3 {
    fn call(&self, a1: i32) -> () {
        unsafe { lambda_3::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_4 {
    a: *mut i32,
    b: *mut i32,
}
impl lambda_4 {
    pub unsafe fn operator_call(&self) -> i32 {
        return ((*self.a) * (*self.b));
    }
}
impl Callable0<i32> for lambda_4 {
    fn call(&self) -> i32 {
        unsafe { lambda_4::operator_call(self) }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
