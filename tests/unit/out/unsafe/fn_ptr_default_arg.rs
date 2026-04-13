extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn identity_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn apply_1(mut x: i32, mut fn_: Option<Option<unsafe fn(i32) -> i32>>) -> i32 {
    let mut fn_: Option<unsafe fn(i32) -> i32> = fn_.unwrap_or(None);
    if !(fn_).is_none() {
        return (unsafe {
            let _arg0: i32 = x;
            (fn_).unwrap()(_arg0)
        });
    }
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            let _fn: Option<unsafe fn(i32) -> i32> = Default::default();
            apply_1(_x, Some(_fn))
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            let _fn: Option<unsafe fn(i32) -> i32> = None;
            apply_1(_x, Some(_fn))
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            let _fn: Option<unsafe fn(i32) -> i32> = Some(identity_0 as _);
            apply_1(_x, Some(_fn))
        }) == (5))
    );
    let mut negate: Option<unsafe fn(i32) -> i32> = Some(|x: i32| {
        return -x;
    });
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            let _fn: Option<unsafe fn(i32) -> i32> = negate;
            apply_1(_x, Some(_fn))
        }) == (-5_i32))
    );
    return 0;
}
