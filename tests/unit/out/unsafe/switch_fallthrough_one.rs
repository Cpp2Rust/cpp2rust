extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fallthrough_one_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 => {
            r += 10;
        }
        v if v == 2 => {
            r += 20;
            break;
        }
        _ => {
            r = -1_i32;
            break;
        }
    });
    return r;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            fallthrough_one_0(_x)
        }) == (30))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            fallthrough_one_0(_x)
        }) == (20))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            fallthrough_one_0(_x)
        }) == (-1_i32))
    );
    return 0;
}
