extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn stacked_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 || v == 2 || v == 3 => {
                r = 100;
                break 'switch;
            }
            v if v == 4 || v == 5 => {
                r = 200;
                break 'switch;
            }
            _ => {
                r = 300;
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
            stacked_0(_x)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 3;
            stacked_0(_x)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            stacked_0(_x)
        }) == (200))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            stacked_0(_x)
        }) == (300))
    );
    return 0;
}
