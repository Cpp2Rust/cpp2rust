extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn stacked_with_inner_fallthrough_0(mut x: i32, mut flag: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 1 || __v == 2 || __v == 3 => {
            if !(flag != 0) {
                r = 50;
                break;
            };
        }
        _ => {
            r = 999;
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
    assert!(((unsafe { stacked_with_inner_fallthrough_0(1, 0,) }) == (50)));
    assert!(((unsafe { stacked_with_inner_fallthrough_0(2, 1,) }) == (999)));
    assert!(((unsafe { stacked_with_inner_fallthrough_0(99, 0,) }) == (999)));
    return 0;
}
