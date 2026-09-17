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
    let mut start: i32 = 5;
    let mut next: lambda_0 = (lambda_0 { start: start });
    assert!(((unsafe { lambda_0::operator_call(&mut next,) }) == (5)));
    assert!(((unsafe { lambda_0::operator_call(&mut next,) }) == (6)));
    assert!(((unsafe { lambda_0::operator_call(&mut next,) }) == (7)));
    assert!(((start) == (5)));
    let mut total: i32 = 0;
    let mut accumulate: lambda_1 = (lambda_1 { total: total });
    assert!(((unsafe { lambda_1::operator_call(&mut accumulate, 1,) }) == (1)));
    assert!(((unsafe { lambda_1::operator_call(&mut accumulate, 2,) }) == (3)));
    assert!(((total) == (0)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {
    start: i32,
}
impl lambda_0 {
    pub unsafe fn operator_call(&mut self) -> i32 {
        return self.start.postfix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    total: i32,
}
impl lambda_1 {
    pub unsafe fn operator_call(&mut self, mut x: i32) -> i32 {
        self.total += x;
        return self.total;
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
