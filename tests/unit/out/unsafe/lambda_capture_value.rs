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
    let mut scale: FnPtr<fn(i32) -> i32> = {
        #[repr(C)]
        struct Captures {
            factor: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
            Captures { factor: factor },
            (|this: &mut Captures, x: i32| unsafe {
                return ((x) * (this.factor));
            }),
        )
    };
    assert!(((unsafe { scale.call(4,) }) == (12)));
    factor = 100;
    assert!(((unsafe { scale.call(4,) }) == (12)));
    let mut slot: i32 = 7;
    let mut p: *mut i32 = (&mut slot as *mut i32);
    let mut read_ptr: FnPtr<fn() -> i32> = {
        #[repr(C)]
        struct Captures {
            p: *mut i32,
        }
        FnPtr::<fn() -> i32>::with_captures_unsafe(
            Captures { p: p },
            (|this: &mut Captures| unsafe {
                return (*this.p);
            }),
        )
    };
    slot = 8;
    assert!(((unsafe { read_ptr.call() }) == (8)));
    let mut s: S = S { x: 1, y: 2 };
    let mut sum: FnPtr<fn() -> i32> = {
        #[repr(C)]
        struct Captures {
            s: S,
        }
        FnPtr::<fn() -> i32>::with_captures_unsafe(
            Captures { s: s },
            (|this: &mut Captures| unsafe {
                return ((this.s.x) + (this.s.y));
            }),
        )
    };
    s.x = 50;
    assert!(((unsafe { sum.call() }) == (3)));
    let mut base: i32 = 10;
    let mut shifted: FnPtr<fn(i32) -> i32> = {
        #[repr(C)]
        struct Captures {
            y: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
            Captures { y: ((base) + (1)) },
            (|this: &mut Captures, x: i32| unsafe {
                return ((x) + (this.y));
            }),
        )
    };
    assert!(((unsafe { shifted.call(5,) }) == (16)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
