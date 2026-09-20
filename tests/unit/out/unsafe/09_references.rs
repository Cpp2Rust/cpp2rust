extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: i32 = 15;
    let h_ref1: *mut i32 = &mut h;
    (*h_ref1) = 16;
    let mut h_ptr: *mut i32 = (h_ref1);
    let h_ref2: *mut i32 = &mut (*h_ptr);
    (*h_ref2) = 17;
    assert!((((*h_ref1) + (*h_ref2)) == (34)));
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let r: *mut i32 = if ((a) < (b)) { &mut a } else { &mut b } as *mut i32;
    (*r) = 10;
    assert!(((a) == (10)));
    let cr: *const i32 = if ((a) > (b)) { &a } else { &b } as *const i32;
    assert!(((*cr) == (10)));
    let x: i32 = 1;
    let y: i32 = 2;
    let cx: *const i32 = if ((x) < (y)) { &x } else { &y } as *const i32;
    assert!(((*cx) == (1)));
    let mut cp: *const i32 = (if ((a) > (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    })
    .cast_const();
    assert!(((*cp) == (10)));
    let mut mp: *mut i32 = if ((a) < (b)) {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    };
    (*mp) = 20;
    assert!(((b) == (20)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
