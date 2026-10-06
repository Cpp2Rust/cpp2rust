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
    pub v: i32,
}
pub unsafe fn operator_comma_0(a: *const S, b: *const S) -> S {
    return S {
        v: ((((*a).v) * (10)) + ((*b).v)),
    };
}
pub unsafe fn operator_literal__k_1(mut v: u64) -> i64 {
    return (((v).wrapping_mul(1000_u64)) as i64);
}
pub unsafe fn operator_literal__half_2(mut v: f64) -> f64 {
    return ((v) / (2_f64));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 3 };
    let mut t: S = S { v: 4 };
    assert!(
        (((unsafe {
            let _a: *const S = &s;
            operator_comma_0(_a, &t)
        })
        .v) == (34))
    );
    assert!(
        (((unsafe {
            let mut _a: S = (unsafe {
                let _a: *const S = &s;
                operator_comma_0(_a, &t)
            });
            let _b: *const S = &s;
            operator_comma_0(&mut _a, _b)
        })
        .v) == (343))
    );
    assert!(((unsafe { operator_literal__k_1(2_u64,) }) == (2000_i64)));
    assert!(((unsafe { operator_literal__half_2(3.0E+0,) }) == (1.5E+0)));
    assert!(((unsafe { operator_literal__k_1(4_u64,) }) == (4000_i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
