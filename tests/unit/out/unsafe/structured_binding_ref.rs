extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Triple {
    pub a: i32,
    pub b: bool,
    pub c: i32,
}
pub unsafe fn sum_0(t: *const Triple) -> i32 {
    let __decomp_1: *const Triple = t;
    return ((((*__decomp_1).a) + (if (*__decomp_1).b { 1 } else { 0 })) + ((*__decomp_1).c));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: Triple = Triple {
        a: 10,
        b: false,
        c: 20,
    };
    let __decomp_2: *mut Triple = &mut t;
    (*__decomp_2).a = 11;
    (*__decomp_2).b = true;
    (*__decomp_2).c += 1;
    assert!(((t.a) == (11)));
    assert!(t.b);
    assert!(((t.c) == (21)));
    t.a = 12;
    assert!((((*__decomp_2).a) == (12)));
    let __decomp_3: *const Triple = &t;
    t.c = 30;
    assert!((((*__decomp_3).a) == (12)));
    assert!((*__decomp_3).b);
    assert!((((*__decomp_3).c) == (30)));
    assert!(((unsafe { sum_0(&t,) }) == (43)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
