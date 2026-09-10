extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn zero_0() -> *mut i32 {
    return std::ptr::null_mut();
}
pub unsafe fn zero_1() -> i64 {
    return 0_i64;
}
pub unsafe fn destroy_2(mut p: *mut i32) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i: i32 = 0_i32;
    let mut d: f64 = 0.0_f64;
    let mut p: *mut i32 = (unsafe { zero_0() });
    assert!(((i) == (0)));
    assert!(((d) == (0.0E+0)));
    assert!((p).is_null());
    assert!(((unsafe { zero_1() }) == (0_i64)));
    let mut x: i32 = 5;
    (unsafe { destroy_2((&mut x as *mut i32)) });
    assert!(((x) == (5)));
    return 0;
}
