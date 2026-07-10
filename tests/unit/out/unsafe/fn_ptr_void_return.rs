extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn negate_0(mut x: *mut i32) {
    (*x) = -(*x);
}
pub unsafe fn zero_out_1(mut x: *mut i32) {
    (*x) = 0;
}
pub unsafe fn run_2(mut fn_: Option<unsafe fn(*mut i32)>, mut x: *mut i32) {
    (unsafe { (fn_).unwrap()(x) });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 42;
    (unsafe { run_2(Some(negate_0), (&mut a as *mut i32)) });
    assert!(((a) == (-42_i32)));
    (unsafe { run_2(Some(zero_out_1), (&mut a as *mut i32)) });
    assert!(((a) == (0)));
    let mut fn_: Option<unsafe fn(*mut i32)> = Some(negate_0);
    assert!(!((fn_).is_none()));
    let mut b: i32 = 10;
    (unsafe { (fn_).unwrap()((&mut b as *mut i32)) });
    assert!(((b) == (-10_i32)));
    return 0;
}
