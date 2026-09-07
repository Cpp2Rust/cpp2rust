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
pub unsafe fn operator_eq_0(a: *const S, b: *const S) -> bool {
    return (((*a).v) == ((*b).v));
}
pub unsafe fn operator_ne_1(a: *const S, b: *const S) -> bool {
    return (((*a).v) != ((*b).v));
}
pub unsafe fn operator_lt_2(a: *const S, b: *const S) -> bool {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn operator_gt_3(a: *const S, b: *const S) -> bool {
    return (((*a).v) > ((*b).v));
}
pub unsafe fn operator_le_4(a: *const S, b: *const S) -> bool {
    return (((*a).v) <= ((*b).v));
}
pub unsafe fn operator_ge_5(a: *const S, b: *const S) -> bool {
    return (((*a).v) >= ((*b).v));
}
pub unsafe fn operator_lt_6(a: *const S, mut b: i32) -> bool {
    return (((*a).v) < (b));
}
pub unsafe fn operator_lt_7(mut a: i32, b: *const S) -> bool {
    return ((a) < ((*b).v));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 1 };
    let mut b: S = S { v: 2 };
    let mut c: S = S { v: 1 };
    assert!(
        (unsafe {
            let _a: *const S = &a as *const S;
            operator_eq_0(_a, &c as *const S)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a as *const S;
            operator_ne_1(_a, &b as *const S)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a as *const S;
            operator_lt_2(_a, &b as *const S)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &b as *const S;
            operator_gt_3(_a, &a as *const S)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a as *const S;
            operator_le_4(_a, &c as *const S)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a as *const S;
            operator_ge_5(_a, &c as *const S)
        })
    );
    assert!(
        !(unsafe {
            let _a: *const S = &b as *const S;
            operator_lt_2(_a, &a as *const S)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &a as *const S;
            operator_lt_6(_a, 5)
        })
    );
    assert!((unsafe { operator_lt_7(0, &a as *const S,) }));
    return 0;
}
