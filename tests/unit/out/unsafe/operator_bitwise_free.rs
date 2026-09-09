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
    pub v: u32,
}
pub unsafe fn operator_bitnot_0(a: *const S) -> S {
    return S { v: !(*a).v };
}
pub unsafe fn operator_bitand_1(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) & ((*b).v)),
    };
}
pub unsafe fn operator_bitor_2(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) | ((*b).v)),
    };
}
pub unsafe fn operator_bitxor_3(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) ^ ((*b).v)),
    };
}
pub unsafe fn operator_shl_4(a: *const S, mut n: i32) -> S {
    return S {
        v: (((*a).v) << (n)),
    };
}
pub unsafe fn operator_shr_5(a: *const S, mut n: i32) -> S {
    return S {
        v: (((*a).v) >> (n)),
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 12_u32 };
    let mut b: S = S { v: 10_u32 };
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_bitnot_0(_a)
        })
        .v) == (!12_u32))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_bitand_1(_a, &b as *const S)
        })
        .v) == (8_u32))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_bitor_2(_a, &b as *const S)
        })
        .v) == (14_u32))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_bitxor_3(_a, &b as *const S)
        })
        .v) == (6_u32))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_shl_4(_a, 2)
        })
        .v) == (48_u32))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_shr_5(_a, 2)
        })
        .v) == (3_u32))
    );
    return 0;
}
