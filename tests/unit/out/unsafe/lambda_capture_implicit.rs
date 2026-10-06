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
    let mut by_value: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let a: i32 = a;
            let b: i32 = b;
            let c: i32 = c;
        },
        |x: i32| -> i32 {
            return ((((a) + (b)) + (c)) + (x));
        },
        copy_from { a: a, b: b, c: c },
        move_from { a: a, b: b, c: c }
    );
    assert!(((unsafe { by_value.call(10,) }) == (16)));
    a = 100;
    assert!(((unsafe { by_value.call(10,) }) == (16)));
    let mut by_ref: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let a: *mut i32 = &mut a;
            let b: *mut i32 = &mut b;
            let c: *mut i32 = &mut c;
        },
        |x: i32| -> i32 {
            return ((((*a) + (*b)) + (*c)) + (x));
        },
        copy_from { a: a, b: b, c: c },
        move_from { a: a, b: b, c: c }
    );
    assert!(((unsafe { by_ref.call(10,) }) == (115)));
    b = 200;
    assert!(((unsafe { by_ref.call(10,) }) == (313)));
    let mut mixed: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let c: *mut i32 = &mut c;
            let a: i32 = a;
            let b: i32 = b;
        },
        |x: i32| -> i32 {
            (*c) += x;
            return (((a) + (b)) + (*c));
        },
        copy_from { c: c, a: a, b: b },
        move_from { c: c, a: a, b: b }
    );
    assert!(((unsafe { mixed.call(1,) }) == (((100) + (200)) + (4))));
    assert!(((c) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
