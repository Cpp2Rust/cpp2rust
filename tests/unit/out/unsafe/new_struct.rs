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
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Triple {
    pub a: i32,
    pub b: i32,
    pub p: Pair,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut Pair = (Box::leak(Box::new(Pair { x: 1, y: 2 })) as *mut Pair);
    let mut out: i32 = (((*p).x) + ((*p).y));
    ::std::mem::drop(Box::from_raw(p));
    assert!(((out) == (3)));
    let mut t: Triple = Triple {
        a: 1,
        b: 0_i32,
        p: <Pair>::default(),
    };
    assert!(((t.a) == (1)));
    assert!(((t.b) == (0)));
    assert!(((t.p.x) == (0)) && ((t.p.y) == (0)));
    let mut q: *mut Triple = (Box::leak(Box::new(Triple {
        a: 2,
        b: 3,
        p: <Pair>::default(),
    })) as *mut Triple);
    assert!((((*q).a) == (2)));
    assert!((((*q).b) == (3)));
    assert!((((*q).p.x) == (0)) && (((*q).p.y) == (0)));
    ::std::mem::drop(Box::from_raw(q));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
