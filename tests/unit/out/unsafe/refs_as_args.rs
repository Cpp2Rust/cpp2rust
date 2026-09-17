extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn more_refs_0(mut x1: i32, mut x2: i32, r1: *mut i32, r2: *const i32) {
    let rx1: *const i32 = &x1;
    let rx2: *mut i32 = &mut x2;
    let mut pr1: *const i32 = (r1).cast_const();
    let mut pr2: *const i32 = (r2);
    let rpr1: *const i32 = &(*pr1);
    let rpr2: *const i32 = &(*pr2);
    let r: *const i32 = r1;
    (*rx2) += ((((((((1) + (*rx1)) + (*rx2)) + (*pr1)) + (*pr2)) + (*rpr1)) + (*rpr2)) + (*r));
    (*r1) = (*rx2);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = 1;
    let x2: i32 = 2;
    (unsafe { more_refs_0(3, 4, &mut x1, &x2) });
    assert!((((x1) + (x2)) == (21)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
