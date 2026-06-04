extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn basic_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    let mut v: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            __v if __v == 0 => {
                r = 10;
                break 'switch;
            }
            __v if __v == 1 => {
                r = 20;
                break 'switch;
            }
            __v if __v == 2 => {
                r = 30;
                break 'switch;
            }
            _ => {
                r = 40;
                break 'switch;
            }
        }
    };
    return r;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { basic_0(0,) }) == (10)));
    assert!(((unsafe { basic_0(2,) }) == (30)));
    assert!(((unsafe { basic_0(99,) }) == (40)));
    return 0;
}
