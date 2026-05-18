extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn no_default_0(mut x: i32) -> i32 {
    let mut r: i32 = -1_i32;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 7 => {
                r = 1;
                break 'switch;
            }
            v if v == 8 => {
                r = 2;
                break 'switch;
            }
            _ => {}
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
            let _x: i32 = 7;
            no_default_0(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 42;
            no_default_0(_x)
        }) == (-1_i32))
    );
    return 0;
}
