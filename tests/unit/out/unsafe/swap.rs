extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn identity_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn swap_by_ptr_1(mut a: *mut i32, mut b: *mut i32) {
    let mut tmp: i32 = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub unsafe fn swap_by_ref_2(a: *mut i32, b: *mut i32) {
    let mut tmp: i32 = (*a);
    (*a) = (*b);
    (*b) = tmp;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut local: i32 = 0;
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = (unsafe {
        let _x: i32 = local;
        identity_0(_x)
    });
    let mut p: *mut i32 = (&mut a as *mut i32);
    p = (&mut ((b) as i32) as *mut i32);
    p = (&mut a as *mut i32);
    (unsafe {
        let _a: *mut i32 = p;
        let _b: *mut i32 = (&mut b as *mut i32);
        swap_by_ptr_1(_a, _b)
    });
    (unsafe {
        let _a: *mut i32 = &mut a as *mut i32;
        let _b: *mut i32 = &mut c as *mut i32;
        swap_by_ref_2(_a, _b)
    });
    return c;
}
