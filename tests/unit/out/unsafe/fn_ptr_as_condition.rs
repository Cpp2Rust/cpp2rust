extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: *mut i32) {
    (*x) *= 2;
}
pub unsafe fn maybe_call_1(mut cb: Option<unsafe fn(*mut i32)>, mut x: *mut i32) {
    if !(cb).is_none() {
        (unsafe {
            let _arg0: *mut i32 = x;
            (cb).unwrap()(_arg0)
        });
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 5;
    (unsafe {
        let _cb: Option<unsafe fn(*mut i32)> = Some(double_it_0);
        let _x: *mut i32 = (&mut a as *mut i32);
        maybe_call_1(_cb, _x)
    });
    assert!(((a) == (10)));
    let mut b: i32 = 5;
    (unsafe {
        let _cb: Option<unsafe fn(*mut i32)> = None;
        let _x: *mut i32 = (&mut b as *mut i32);
        maybe_call_1(_cb, _x)
    });
    assert!(((b) == (5)));
    let mut fn_: Option<unsafe fn(*mut i32)> = None;
    if !!(fn_).is_none() {
        fn_ = Some(double_it_0);
    }
    let mut c: i32 = 3;
    if !(fn_).is_none() {
        (unsafe {
            let _arg0: *mut i32 = (&mut c as *mut i32);
            (fn_).unwrap()(_arg0)
        });
    }
    assert!(((c) == (6)));
    return 0;
}
