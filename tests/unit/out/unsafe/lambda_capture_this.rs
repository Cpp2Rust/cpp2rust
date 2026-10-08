extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub n: i32,
    pub step: i32,
}
impl S {
    pub unsafe fn add(&mut self, mut k: i32) {
        self.n += k;
    }
    pub unsafe fn scaled(&self) -> i32 {
        return ((self.n) * (self.step));
    }
    pub unsafe fn bump(&mut self, mut by: i32) {
        let mut inc: FnPtr<fn(i32)> = lambda_unsafe!(
            {
                let this_: *mut S = (self as *mut S);
            },
            |k: i32| {
                (*this_).n += k;
            }
        );
        (unsafe { inc.call(by) });
        (unsafe { inc.call(by) });
    }
    pub unsafe fn bump_via_method(&mut self, mut by: i32) {
        let mut inc: FnPtr<fn(i32)> = lambda_unsafe!(
            {
                let this_: *mut S = (self as *mut S);
            },
            |k: i32| {
                (unsafe { S::add(&mut (*this_), k) });
            }
        );
        (unsafe { inc.call(by) });
    }
    pub unsafe fn read_scaled(&self) -> i32 {
        let mut get: FnPtr<fn() -> i32> = lambda_unsafe!(
            {
                let this_: *const S = (self as *const S);
            },
            || -> i32 {
                return (unsafe { S::scaled(&(*this_)) });
            }
        );
        return (unsafe { get.call() });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { n: 0, step: 2 };
    (unsafe { S::bump(&mut s, 3) });
    assert!(((s.n) == (6)));
    (unsafe { S::bump_via_method(&mut s, 4) });
    assert!(((s.n) == (10)));
    assert!(((unsafe { S::read_scaled(&s,) }) == (20)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
