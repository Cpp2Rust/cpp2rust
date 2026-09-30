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
pub struct S {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}
pub unsafe fn bump_0(mut s: *mut S) -> i32 {
    (*s).b += 10;
    return (*s).b;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: *mut S = (libcc2rs::calloc_unsafe(1_usize, ::std::mem::size_of::<S>()) as *mut S);
    assert!(!((s).is_null()));
    (*s).b = 1;
    (*s).a = (unsafe { bump_0(s) });
    assert!((((*s).a) == (11)));
    assert!((((*s).b) == (11)));
    (*s).a = 1;
    (*s).b = 2;
    (*s).c = 0;
    if (((*s).a) < ((*s).b)) && (((*s).c.postfix_inc()) == (0)) {
        (*s).a = 5;
    }
    assert!((((*s).a) == (5)) && (((*s).c) == (1)));
    if (((*s).a) < ((*s).b)) && (((*s).c.postfix_inc()) == (0)) {
        (*s).a = 6;
    }
    assert!((((*s).a) == (5)) && (((*s).c) == (1)));
    let mut x: i32 = (((*s).a)
        + ({
            (*s).b = 3;
            (*s).b
        }));
    assert!(((x) == (8)) && (((*s).b) == (3)));
    let mut y: i32 = 0;
    (*s).c = ({
        y = 99;
        y
    });
    assert!((((*s).c) == (99)) && ((y) == (99)));
    (*s).a += (unsafe { bump_0(s) });
    assert!(((((*s).a) == (18)) && (((*s).b) == (13))) && (((*s).c) == (99)));
    libcc2rs::free_unsafe((s as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
