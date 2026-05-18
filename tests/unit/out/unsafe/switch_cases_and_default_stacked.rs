extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cases_and_default_stacked_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 3 => {
                r = 3;
                break 'switch;
            }
            _ => {
                r = 42;
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
            cases_and_default_stacked_0(_x)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            cases_and_default_stacked_0(_x)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 3;
            cases_and_default_stacked_0(_x)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            cases_and_default_stacked_0(_x)
        }) == (42))
    );
    return 0;
}
