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
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut factor: i32 = 3;
    let mut scale: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let factor: i32 = factor;
        },
        |x: i32| -> i32 {
            return ((x) * (factor));
        }
    );
    assert!(((unsafe { scale.call(4,) }) == (12)));
    factor = 100;
    assert!(((unsafe { scale.call(4,) }) == (12)));
    let mut slot: i32 = 7;
    let mut p: *mut i32 = (&mut slot as *mut i32);
    let mut read_ptr: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let p: *mut i32 = p;
        },
        || -> i32 {
            return (*p);
        }
    );
    slot = 8;
    assert!(((unsafe { read_ptr.call() }) == (8)));
    let mut s: S = S { x: 1, y: 2 };
    let mut sum: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let s: S = s;
        },
        || -> i32 {
            return ((s.x) + (s.y));
        }
    );
    s.x = 50;
    assert!(((unsafe { sum.call() }) == (3)));
    let mut base: i32 = 10;
    let mut shifted: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let y: i32 = ((base) + (1));
        },
        |x: i32| -> i32 {
            return ((x) + (y));
        }
    );
    assert!(((unsafe { shifted.call(5,) }) == (16)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
