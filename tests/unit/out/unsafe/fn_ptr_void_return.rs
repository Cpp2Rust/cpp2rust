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
    (unsafe {
        let _arg0: *mut i32 = x;
        (fn_).unwrap()(_arg0)
    });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 42;
    (unsafe {
        let _fn: Option<unsafe fn(*mut i32)> = Some(negate_0);
        let _x: *mut i32 = (&mut a as *mut i32);
        run_2(_fn, _x)
    });
    assert!(((a) == (-42_i32)));
    (unsafe {
        let _fn: Option<unsafe fn(*mut i32)> = Some(zero_out_1);
        let _x: *mut i32 = (&mut a as *mut i32);
        run_2(_fn, _x)
    });
    assert!(((a) == (0)));
    let mut fn_: Option<unsafe fn(*mut i32)> = Some(negate_0);
    assert!(!((fn_).is_none()));
    let mut b: i32 = 10;
    (unsafe {
        let _arg0: *mut i32 = (&mut b as *mut i32);
        (fn_).unwrap()(_arg0)
    });
    assert!(((b) == (-10_i32)));
    return 0;
}
