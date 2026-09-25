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
    pub v: i32,
}
impl S {
    pub unsafe fn nested_this(&mut self) -> i32 {
        return (unsafe {
            (|y: i32| {
                return (unsafe {
                    (|z: i32| {
                        return (((self.v) + (y)) + (z));
                    })(1)
                });
            })(20)
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
    let mut x: i32 = 10;
    assert!(
        ((unsafe {
            (|y: i32| {
                return (unsafe {
                    (|z: i32| {
                        return (((x) + (y)) + (z));
                    })(1)
                });
            })(20)
        }) == (31))
    );
    x = 100;
    assert!(
        ((unsafe {
            (|y: i32| {
                return (unsafe {
                    (|z: i32| {
                        return (((x) + (y)) + (z));
                    })(1)
                });
            })(20)
        }) == (121))
    );
    let mut s: S = S { v: 5 };
    assert!(((unsafe { S::nested_this(&mut s,) }) == (26)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
