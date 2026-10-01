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
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 3;
    let mut by_value: FnPtr<fn(i32) -> i32> = {
        #[repr(C)]
        struct Captures {
            a: i32,
            b: i32,
            c: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
            Captures { a: a, b: b, c: c },
            (|this: &mut Captures, x: i32| unsafe {
                return ((((this.a) + (this.b)) + (this.c)) + (x));
            }),
        )
    };
    assert!(((unsafe { by_value.call(10,) }) == (16)));
    a = 100;
    assert!(((unsafe { by_value.call(10,) }) == (16)));
    let mut by_ref: FnPtr<fn(i32) -> i32> = {
        #[repr(C)]
        struct Captures {
            a: *mut i32,
            b: *mut i32,
            c: *mut i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
            Captures {
                a: &mut a,
                b: &mut b,
                c: &mut c,
            },
            (|this: &mut Captures, x: i32| unsafe {
                return ((((*this.a) + (*this.b)) + (*this.c)) + (x));
            }),
        )
    };
    assert!(((unsafe { by_ref.call(10,) }) == (115)));
    b = 200;
    assert!(((unsafe { by_ref.call(10,) }) == (313)));
    let mut mixed: FnPtr<fn(i32) -> i32> = {
        #[repr(C)]
        struct Captures {
            c: *mut i32,
            a: i32,
            b: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
            Captures {
                c: &mut c,
                a: a,
                b: b,
            },
            (|this: &mut Captures, x: i32| unsafe {
                (*this.c) += x;
                return (((this.a) + (this.b)) + (*this.c));
            }),
        )
    };
    assert!(((unsafe { mixed.call(1,) }) == (((100) + (200)) + (4))));
    assert!(((c) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
