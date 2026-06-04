extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fallthrough_default_0(mut x: i32, mut flag: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 7 => {
            if (flag != 0) {
                r = 100;
                break;
            };
        }
        _ => {
            r = 42;
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
    assert!(((unsafe { fallthrough_default_0(7, 0,) }) == (42)));
    assert!(((unsafe { fallthrough_default_0(7, 1,) }) == (100)));
    assert!(((unsafe { fallthrough_default_0(99, 0,) }) == (42)));
    return 0;
}
