extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn identity_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn apply_1(mut x: i32, mut fn_: Option<Option<unsafe fn(i32) -> i32>>) -> i32 {
    let mut fn_: Option<unsafe fn(i32) -> i32> = fn_.unwrap_or(None);
    if !(fn_).is_none() {
        return (unsafe { (fn_).unwrap()(x) });
    }
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { apply_1(5, None,) }) == (5)));
    assert!(((unsafe { apply_1(5, Some(None),) }) == (5)));
    assert!(((unsafe { apply_1(5, Some(Some(identity_0)),) }) == (5)));
    let mut negate: Option<unsafe fn(i32) -> i32> = Some(|a1: i32| {
        let __this: lambda_2 = lambda_2 {};
        unsafe { lambda_2::operator_call(&__this, a1) }
    });
    assert!(((unsafe { apply_1(5, Some(negate),) }) == (-5_i32)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_2 {}
impl lambda_2 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return -x;
    }
}
impl Callable1<i32, i32> for lambda_2 {
    fn call(&self, a1: i32) -> i32 {
        let __this: lambda_2 = self.clone();
        unsafe { lambda_2::operator_call(&__this, a1) }
    }
}
