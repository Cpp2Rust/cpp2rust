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
pub struct Counter {
    pub n: i32,
}
impl Counter {
    pub unsafe fn bump(&mut self, mut by: i32) {
        let mut inc: lambda_0 = (lambda_0 {
            this_: (self as *mut Counter),
        });
        (unsafe { lambda_0::operator_call(&inc, by) });
        (unsafe { lambda_0::operator_call(&inc, by) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {
    this_: *mut Counter,
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
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Counter = <Counter>::default();
    (unsafe { Counter::bump(&mut c, 3) });
    assert!(((c.n) == (6)));
    return 0;
}
