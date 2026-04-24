extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn compound_case_body_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 => {
                let mut y: i32 = 10;
                let mut z: i32 = 20;
                r = ((y) + (z));
                break 'switch;
            }
            v if v == 2 => {
                let mut y: i32 = 100;
                r = ((y) - (1));
                break 'switch;
            }
            _ => {
                r = -1_i32;
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
            compound_case_body_0(_x)
        }) == (30))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            compound_case_body_0(_x)
        }) == (99))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            compound_case_body_0(_x)
        }) == (-1_i32))
    );
    return 0;
}
