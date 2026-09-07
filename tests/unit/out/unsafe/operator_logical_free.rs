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
pub unsafe fn operator_not_0(a: *const S) -> bool {
    return (((*a).v) == (0));
}
pub unsafe fn operator_and_1(a: *const S, b: *const S) -> bool {
    return (((*a).v) != (0)) && (((*b).v) != (0));
}
pub unsafe fn operator_or_2(a: *const S, b: *const S) -> bool {
    return (((*a).v) != (0)) || (((*b).v) != (0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: S = S { v: 1 };
    let mut f: S = S { v: 0 };
    assert!(
        (unsafe {
            let _a: *const S = &f as *const S;
            operator_not_0(_a)
        })
    );
    assert!(
        (!(unsafe {
            let _a: *const S = &t as *const S;
            operator_not_0(_a)
        }))
    );
    assert!(
        (unsafe {
            let _a: *const S = &t as *const S;
            let _b: *const S = &t as *const S;
            operator_and_1(_a, _b)
        })
    );
    assert!(
        (!(unsafe {
            let _a: *const S = &t as *const S;
            operator_and_1(_a, &f as *const S)
        }))
    );
    assert!(
        (unsafe {
            let _a: *const S = &t as *const S;
            operator_or_2(_a, &f as *const S)
        })
    );
    assert!(
        (!(unsafe {
            let _a: *const S = &f as *const S;
            let _b: *const S = &f as *const S;
            operator_or_2(_a, _b)
        }))
    );
    return 0;
}
