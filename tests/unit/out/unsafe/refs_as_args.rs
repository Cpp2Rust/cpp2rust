extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn more_refs_0(mut x1: i32, mut x2: i32, r1: *mut i32, r2: *const i32) {
    let rx1: *const i32 = &x1 as *const i32;
    let rx2: *mut i32 = &mut x2 as *mut i32;
    let mut pr1: *const i32 = (r1);
    let mut pr2: *const i32 = (r2);
    let rpr1: *const i32 = &(*pr1) as *const i32;
    let rpr2: *const i32 = &(*pr2) as *const i32;
    let r: *const i32 = r1;
    (*rx2) += ((((((((1) + (*rx1)) + (*rx2)) + (*pr1)) + (*pr2)) + (*rpr1)) + (*rpr2)) + (*r));
    (*r1) = (*rx2);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = 1;
    let x2: i32 = 2;
    (unsafe {
        let _x1: i32 = 3;
        let _x2: i32 = 4;
        let _r1: *mut i32 = &mut x1 as *mut i32;
        let _r2: *const i32 = &x2 as *const i32;
        more_refs_0(_x1, _x2, _r1, _r2)
    });
    return ((x1) + (x2));
}
