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
pub unsafe fn operator_add_0(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) + ((*b).v)),
    };
}
pub unsafe fn operator_sub_1(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) - ((*b).v)),
    };
}
pub unsafe fn operator_mul_2(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) * ((*b).v)),
    };
}
pub unsafe fn operator_div_3(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) / ((*b).v)),
    };
}
pub unsafe fn operator_rem_4(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) % ((*b).v)),
    };
}
pub unsafe fn operator_pos_5(a: *const S) -> S {
    return S { v: (*a).v };
}
pub unsafe fn operator_neg_6(a: *const S) -> S {
    return S { v: -(*a).v };
}
pub unsafe fn operator_inc_7(a: *mut S) -> *mut S {
    (*a).v.prefix_inc();
    return a;
}
pub unsafe fn operator_post_inc_8(a: *mut S, mut _a1: i32) -> S {
    let mut old: S = (*a);
    (*a).v.prefix_inc();
    return old;
}
pub unsafe fn operator_dec_9(a: *mut S) -> *mut S {
    (*a).v.prefix_dec();
    return a;
}
pub unsafe fn operator_post_dec_10(a: *mut S, mut _a1: i32) -> S {
    let mut old: S = (*a);
    (*a).v.prefix_dec();
    return old;
}
pub unsafe fn operator_add_11(a: *const S, mut b: i32) -> S {
    return S {
        v: (((*a).v) + (b)),
    };
}
pub unsafe fn operator_add_12(mut a: i32, b: *const S) -> S {
    return S {
        v: ((a) + ((*b).v)),
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 7 };
    let mut b: S = S { v: 2 };
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_add_0(_a, &b as *const S)
        })
        .v) == (9))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_sub_1(_a, &b as *const S)
        })
        .v) == (5))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_mul_2(_a, &b as *const S)
        })
        .v) == (14))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_div_3(_a, &b as *const S)
        })
        .v) == (3))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_rem_4(_a, &b as *const S)
        })
        .v) == (1))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_pos_5(_a)
        })
        .v) == (7))
    );
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_neg_6(_a)
        })
        .v) == (-7_i32))
    );
    assert!(
        (((*(unsafe {
            let _a: *mut S = &mut a as *mut S;
            operator_inc_7(_a)
        }))
        .v) == (8))
    );
    assert!(
        (((unsafe {
            let _a: *mut S = &mut a as *mut S;
            operator_post_inc_8(_a, 0)
        })
        .v) == (8))
    );
    assert!(((a.v) == (9)));
    assert!(
        (((*(unsafe {
            let _a: *mut S = &mut a as *mut S;
            operator_dec_9(_a)
        }))
        .v) == (8))
    );
    assert!(
        (((unsafe {
            let _a: *mut S = &mut a as *mut S;
            operator_post_dec_10(_a, 0)
        })
        .v) == (8))
    );
    assert!(((a.v) == (7)));
    assert!(
        (((unsafe {
            let _a: *const S = &a as *const S;
            operator_add_11(_a, 1)
        })
        .v) == (8))
    );
    assert!((((unsafe { operator_add_12(1, &a as *const S,) }).v) == (8)));
    return 0;
}
