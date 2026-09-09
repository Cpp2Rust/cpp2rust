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
pub unsafe fn operator_add_assign_0(a: *mut S, b: *const S) -> *mut S {
    (*(a)).v = ((*(a)).v).wrapping_add((*(b)).v);
    return a;
}
pub unsafe fn operator_sub_assign_1(a: *mut S, b: *const S) -> *mut S {
    (*(a)).v = ((*(a)).v).wrapping_sub((*(b)).v);
    return a;
}
pub unsafe fn operator_mul_assign_2(a: *mut S, b: *const S) -> *mut S {
    (*(a)).v = ((*(a)).v).wrapping_mul((*(b)).v);
    return a;
}
pub unsafe fn operator_div_assign_3(a: *mut S, b: *const S) -> *mut S {
    (*(a)).v = ((*(a)).v).wrapping_div((*(b)).v);
    return a;
}
pub unsafe fn operator_rem_assign_4(a: *mut S, b: *const S) -> *mut S {
    (*(a)).v = ((*(a)).v).wrapping_rem((*(b)).v);
    return a;
}
pub unsafe fn operator_bitand_assign_5(a: *mut S, b: *const S) -> *mut S {
    (*(a)).v &= (*(b)).v;
    return a;
}
pub unsafe fn operator_bitor_assign_6(a: *mut S, b: *const S) -> *mut S {
    (*(a)).v |= (*(b)).v;
    return a;
}
pub unsafe fn operator_bitxor_assign_7(a: *mut S, b: *const S) -> *mut S {
    (*(a)).v ^= (*(b)).v;
    return a;
}
pub unsafe fn operator_shl_assign_8(a: *mut S, mut n: i32) -> *mut S {
    (*(a)).v <<= n;
    return a;
}
pub unsafe fn operator_shr_assign_9(a: *mut S, mut n: i32) -> *mut S {
    (*(a)).v >>= n;
    return a;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 6_u32 };
    let mut b: S = S { v: 4_u32 };
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_add_assign_0(_a, &b as *const S)
    });
    assert!(((a.v) == (10_u32)));
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_sub_assign_1(_a, &b as *const S)
    });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_mul_assign_2(_a, &b as *const S)
    });
    assert!(((a.v) == (24_u32)));
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_div_assign_3(_a, &b as *const S)
    });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_rem_assign_4(_a, &b as *const S)
    });
    assert!(((a.v) == (2_u32)));
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_bitor_assign_6(_a, &b as *const S)
    });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_bitand_assign_5(_a, &b as *const S)
    });
    assert!(((a.v) == (4_u32)));
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_bitxor_assign_7(_a, &b as *const S)
    });
    assert!(((a.v) == (0_u32)));
    a.v = 3_u32;
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_shl_assign_8(_a, 2)
    });
    assert!(((a.v) == (12_u32)));
    (unsafe {
        let _a: *mut S = &mut a as *mut S;
        operator_shr_assign_9(_a, 1)
    });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _a: *mut S = &mut (*(unsafe {
            let _a: *mut S = &mut a as *mut S;
            operator_add_assign_0(_a, &b as *const S)
        })) as *mut S;
        let _b: *const S = &b as *const S;
        operator_add_assign_0(_a, _b)
    });
    assert!(((a.v) == (14_u32)));
    return 0;
}
