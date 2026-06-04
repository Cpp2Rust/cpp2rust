extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn default_first_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            __v if __v == 1 => {
                r = 1;
                break 'switch;
            }
            __v if __v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = 7;
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
    assert!(((unsafe { default_first_0(1,) }) == (1)));
    assert!(((unsafe { default_first_0(99,) }) == (7)));
    return 0;
}
