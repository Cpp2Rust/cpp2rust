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
    pub n: i32,
}
impl S {
    pub unsafe fn twice(&self) -> i32 {
        return ((self.n) * (2));
    }
    pub unsafe fn modify_copy(&mut self) -> i32 {
        let mut r: i32 = (unsafe {
            (|| {
                self.n += 10;
                return self.n;
            })()
        });
        return (((r) * (100)) + (self.n));
    }
    pub unsafe fn snapshot(&mut self) -> i32 {
        self.n = 99;
        return (unsafe {
            (|| {
                return (unsafe { S::twice(self) });
            })()
        });
    }
    pub unsafe fn mixed(&mut self, mut k: i32) -> i32 {
        self.n = 0;
        return (unsafe {
            (|| {
                return ((self.n) + (k));
            })()
        });
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
