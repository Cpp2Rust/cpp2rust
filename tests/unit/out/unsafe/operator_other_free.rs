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
pub unsafe fn operator_comma_0(a: *const S, b: *const S) -> S {
    return S {
        v: ((((*(a)).v) * (10)) + ((*(b)).v)),
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 3 };
    let mut t: S = S { v: 4 };
    assert!(
        (((unsafe {
            let _a: *const S = &s as *const S;
            operator_comma_0(_a, &t as *const S)
        })
        .v) == (34))
    );
    assert!(
        (((unsafe {
            let mut _a: S = (unsafe {
                let _a: *const S = &s as *const S;
                operator_comma_0(_a, &t as *const S)
            });
            let _b: *const S = &s as *const S;
            operator_comma_0(&mut _a, _b)
        })
        .v) == (343))
    );
    return 0;
}
