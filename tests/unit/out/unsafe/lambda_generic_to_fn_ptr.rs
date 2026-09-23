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
    let mut negate: lambda_0 = (lambda_0 {});
    let mut fi: Option<unsafe fn(i32) -> i32> = (unsafe { negate.to_free_function_int_const() });
    let mut fd: Option<unsafe fn(f64) -> f64> = (unsafe { negate.to_free_function_double_const() });
    assert!(((unsafe { (fi).unwrap()(3,) }) == (-3_i32)));
    assert!(((unsafe { (fd).unwrap()(1.5E+0,) }) == (-1.5E+0)));
    let mut square: lambda_1 = (lambda_1 {});
    let mut si: Option<unsafe fn(i32) -> i32> = (unsafe { square.to_free_function_int_const() });
    let mut sd: Option<unsafe fn(f64) -> f64> = (unsafe { square.to_free_function_double_const() });
    assert!(((unsafe { (si).unwrap()(3,) }) == (9)));
    assert!(((unsafe { (sd).unwrap()(1.5E+0,) }) == (2.25E+0)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {}
impl lambda_0 {
    pub unsafe fn operator_call_i32__int_const(mut x: i32) -> i32 {
        return -x;
    }
    pub unsafe fn operator_call_f64__double_const(mut x: f64) -> f64 {
        return -x;
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_0::operator_call_i32__int_const(a1) }
    }
}
impl Callable1<f64, f64> for lambda_0 {
    fn call(&self, a1: f64) -> f64 {
        unsafe { lambda_0::operator_call_f64__double_const(a1) }
    }
}
impl lambda_0 {
    pub fn to_free_function_int_const(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_0::operator_call_i32__int_const)
    }
}
impl lambda_0 {
    pub fn to_free_function_double_const(&self) -> Option<unsafe fn(f64) -> f64> {
        Some(lambda_0::operator_call_f64__double_const)
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {}
impl lambda_1 {
    pub unsafe fn operator_call_i32__int_const(mut x: i32) -> i32 {
        return ((x) * (x));
    }
    pub unsafe fn operator_call_f64__double_const(mut x: f64) -> f64 {
        return ((x) * (x));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_1::operator_call_i32__int_const(a1) }
    }
}
impl Callable1<f64, f64> for lambda_1 {
    fn call(&self, a1: f64) -> f64 {
        unsafe { lambda_1::operator_call_f64__double_const(a1) }
    }
}
impl lambda_1 {
    pub fn to_free_function_int_const(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_1::operator_call_i32__int_const)
    }
}
impl lambda_1 {
    pub fn to_free_function_double_const(&self) -> Option<unsafe fn(f64) -> f64> {
        Some(lambda_1::operator_call_f64__double_const)
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
