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
        __v if __v == 1 => {
            r += 10;
        }
        __v if __v == 2 => {
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
    assert!(((unsafe { fallthrough_one_0(1,) }) == (30)));
    assert!(((unsafe { fallthrough_one_0(2,) }) == (20)));
    assert!(((unsafe { fallthrough_one_0(99,) }) == (-1_i32)));
    return 0;
}
