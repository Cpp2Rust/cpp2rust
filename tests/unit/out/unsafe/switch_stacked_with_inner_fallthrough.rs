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
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 || v == 2 || v == 3 => {
                if !(flag != 0) {
                    r = 50;
                    break 'switch;
                };
            }
            _ => {
                r = 999;
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
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            let _flag: i32 = 0;
            stacked_with_inner_fallthrough_0(_x, _flag)
        }) == (50))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            let _flag: i32 = 1;
            stacked_with_inner_fallthrough_0(_x, _flag)
        }) == (999))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            let _flag: i32 = 0;
            stacked_with_inner_fallthrough_0(_x, _flag)
        }) == (999))
    );
    return 0;
}
