extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn smaller_0(x1: *mut i32, x2: *mut i32) -> *mut i32 {
    return if (((*x1) < (*x2)) as bool) {
        (x1)
    } else {
        (x2)
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut out: *mut i32 = Default::default();
    let mut x1: i32 = 1;
    if (x1 != 0) {
        let mut x2: i32 = -1_i32;
        out = (unsafe {
            let _x1: *mut i32 = &mut x1 as *mut i32;
            let _x2: *mut i32 = &mut x2 as *mut i32;
            smaller_0(_x1, _x2)
        });
    }
    return (*out);
}
