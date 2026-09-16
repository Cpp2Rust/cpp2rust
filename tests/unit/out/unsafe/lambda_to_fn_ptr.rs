extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_0(mut x: i32, mut fn_: Option<unsafe fn(i32) -> i32>) -> i32 {
    return (unsafe { (fn_).unwrap()(x) });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fresh: Option<unsafe fn(i32) -> i32> = Some(lambda_1::operator_call);
    assert!(((unsafe { (fresh).unwrap()(5,) }) == (-5_i32)));
    let mut twice: lambda_2 = (lambda_2 {});
    let mut named: Option<unsafe fn(i32) -> i32> = Some(lambda_2::operator_call);
    assert!(((unsafe { (named).unwrap()(5,) }) == (10)));
    assert!(((unsafe { apply_0(5, Some(lambda_2::operator_call),) }) == (10)));
    named = fresh;
    assert!(((unsafe { (named).unwrap()(3,) }) == (-3_i32)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {}
impl lambda_1 {
    pub unsafe fn operator_call(mut x: i32) -> i32 {
        return -x;
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_1::operator_call(a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {}
impl lambda_2 {
    pub unsafe fn operator_call(mut x: i32) -> i32 {
        return ((x) * (2));
    }
}
impl Callable1<i32, i32> for lambda_2 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_2::operator_call(a1) }
    }
}
