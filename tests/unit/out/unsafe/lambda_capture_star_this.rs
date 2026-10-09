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
}
impl S {
    pub unsafe fn twice(&self) -> i32 {
        return ((self.n) * (2));
    }
    pub unsafe fn modify_copy(&mut self) -> i32 {
        let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
            {
                let this_: S = (*(self as *mut S));
            },
            || -> i32 {
                (*(&raw mut this_)).n += 10;
                return (*(&raw mut this_)).n;
            },
        );
        let mut r: i32 = (unsafe { f.call() });
        return (((r) * (100)) + (self.n));
    }
    pub unsafe fn snapshot(&mut self) -> i32 {
        let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
            {
                let this_: S = (*(self as *mut S));
            },
            || -> i32 {
                return (unsafe { S::twice(&(*(&raw mut this_))) });
            },
        );
        self.n = 99;
        return (unsafe { f.call() });
    }
    pub unsafe fn mixed(&mut self, mut k: i32) -> i32 {
        let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
            {
                let this_: S = (*(self as *mut S));
                let k: i32 = k;
            },
            || -> i32 {
                return (((*(&raw mut this_)).n) + (k));
            },
        );
        self.n = 0;
        return (unsafe { f.call() });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { n: 1 };
    assert!(((unsafe { S::modify_copy(&mut s,) }) == (1101)));
    assert!(((s.n) == (1)));
    assert!(((unsafe { S::snapshot(&mut s,) }) == (2)));
    assert!(((s.n) == (99)));
    assert!(((unsafe { S::mixed(&mut s, 1,) }) == (100)));
    assert!(((s.n) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
