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
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut factor: i32 = 3;
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((x) * (factor));
            })(4)
        }) == (12))
    );
    factor = 100;
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((x) * (factor));
            })(4)
        }) == (12))
    );
    let mut slot: i32 = 7;
    let mut p: *mut i32 = (&mut slot as *mut i32);
    slot = 8;
    assert!(
        ((unsafe {
            (|| {
                return (*p);
            })()
        }) == (8))
    );
    let mut s: S = S { x: 1, y: 2 };
    s.x = 50;
    assert!(
        ((unsafe {
            (|| {
                return ((s.x) + (s.y));
            })()
        }) == (3))
    );
    let mut base: i32 = 10;
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((x) + (y));
            })(5)
        }) == (16))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
