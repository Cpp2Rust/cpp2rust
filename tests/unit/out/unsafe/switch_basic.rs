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
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 0 => {
                r = 10;
                break 'switch;
            }
            v if v == 1 => {
                r = 20;
                break 'switch;
            }
            v if v == 2 => {
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
    assert!(
        ((unsafe {
            let _x: i32 = 0;
            basic_0(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            basic_0(_x)
        }) == (30))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            basic_0(_x)
        }) == (40))
    );
    return 0;
}
