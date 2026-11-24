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
    pub v: u32,
}
pub unsafe fn operator_add_assign_0(a: *mut S, b: *const S) -> *mut S {
    match ({ (*b).v }, &mut ((*a).v)) {
        (__tmp_0, __tmp_1) => {
            (*__tmp_1) = (((*__tmp_1) as u32).wrapping_add((__tmp_0 as u32))) as u32;
        }
    };
    return a;
}
pub unsafe fn operator_sub_assign_1(a: *mut S, b: *const S) -> *mut S {
    match ({ (*b).v }, &mut ((*a).v)) {
        (__tmp_2, __tmp_3) => {
            (*__tmp_3) = (((*__tmp_3) as u32).wrapping_sub((__tmp_2 as u32))) as u32;
        }
    };
    return a;
}
pub unsafe fn operator_mul_assign_2(a: *mut S, b: *const S) -> *mut S {
    match ({ (*b).v }, &mut ((*a).v)) {
        (__tmp_4, __tmp_5) => {
            (*__tmp_5) = (((*__tmp_5) as u32).wrapping_mul((__tmp_4 as u32))) as u32;
        }
    };
    return a;
}
pub unsafe fn operator_div_assign_3(a: *mut S, b: *const S) -> *mut S {
    match ({ (*b).v }, &mut ((*a).v)) {
        (__tmp_6, __tmp_7) => {
            (*__tmp_7) = (((*__tmp_7) as u32).wrapping_div((__tmp_6 as u32))) as u32;
        }
    };
    return a;
}
pub unsafe fn operator_rem_assign_4(a: *mut S, b: *const S) -> *mut S {
    match ({ (*b).v }, &mut ((*a).v)) {
        (__tmp_8, __tmp_9) => {
            (*__tmp_9) = (((*__tmp_9) as u32).wrapping_rem((__tmp_8 as u32))) as u32;
        }
    };
    return a;
}
pub unsafe fn operator_bitand_assign_5(a: *mut S, b: *const S) -> *mut S {
    (*a).v &= (*b).v;
    return a;
}
pub unsafe fn operator_bitor_assign_6(a: *mut S, b: *const S) -> *mut S {
    (*a).v |= (*b).v;
    return a;
}
pub unsafe fn operator_bitxor_assign_7(a: *mut S, b: *const S) -> *mut S {
    (*a).v ^= (*b).v;
    return a;
}
pub unsafe fn operator_shl_assign_8(a: *mut S, mut n: i32) -> *mut S {
    (*a).v <<= n;
    return a;
}
pub unsafe fn operator_shr_assign_9(a: *mut S, mut n: i32) -> *mut S {
    (*a).v >>= n;
    return a;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 6_u32 };
    let mut b: S = S { v: 4_u32 };
    (unsafe {
        let _a: *mut S = &mut a;
        operator_add_assign_0(_a, &b)
    });
    assert!(((a.v) == (10_u32)));
    (unsafe {
        let _a: *mut S = &mut a;
        operator_sub_assign_1(_a, &b)
    });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _a: *mut S = &mut a;
        operator_mul_assign_2(_a, &b)
    });
    assert!(((a.v) == (24_u32)));
    (unsafe {
        let _a: *mut S = &mut a;
        operator_div_assign_3(_a, &b)
    });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _a: *mut S = &mut a;
        operator_rem_assign_4(_a, &b)
    });
    assert!(((a.v) == (2_u32)));
    (unsafe {
        let _a: *mut S = &mut a;
        operator_bitor_assign_6(_a, &b)
    });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _a: *mut S = &mut a;
        operator_bitand_assign_5(_a, &b)
    });
    assert!(((a.v) == (4_u32)));
    (unsafe {
        let _a: *mut S = &mut a;
        operator_bitxor_assign_7(_a, &b)
    });
    assert!(((a.v) == (0_u32)));
    a.v = 3_u32;
    (unsafe {
        let _a: *mut S = &mut a;
        operator_shl_assign_8(_a, 2)
    });
    assert!(((a.v) == (12_u32)));
    (unsafe {
        let _a: *mut S = &mut a;
        operator_shr_assign_9(_a, 1)
    });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _a: *mut S = &mut (*(unsafe {
            let _a: *mut S = &mut a;
            operator_add_assign_0(_a, &b)
        }));
        let _b: *const S = &b;
        operator_add_assign_0(_a, _b)
    });
    assert!(((a.v) == (14_u32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
