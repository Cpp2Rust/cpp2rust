extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn empty_case_with_break_0(mut x: i32) -> i32 {
    let mut r: i32 = 5;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            __v if __v == 1 => {
                break 'switch;
            }
            __v if __v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = 9;
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
    assert!(((unsafe { empty_case_with_break_0(1,) }) == (5)));
    assert!(((unsafe { empty_case_with_break_0(2,) }) == (2)));
    assert!(((unsafe { empty_case_with_break_0(9,) }) == (9)));
    return 0;
}
