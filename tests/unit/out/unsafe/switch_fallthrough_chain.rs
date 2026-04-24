extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fallthrough_chain_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 => {
                r += 1;
            }
            v if v == 2 => {
                r += 2;
            }
            v if v == 3 => {
                r += 4;
            }
            v if v == 4 => {
                r += 8;
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
            fallthrough_chain_0(_x)
        }) == (15))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            fallthrough_chain_0(_x)
        }) == (14))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 3;
            fallthrough_chain_0(_x)
        }) == (12))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 4;
            fallthrough_chain_0(_x)
        }) == (8))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            fallthrough_chain_0(_x)
        }) == (-1_i32))
    );
    return 0;
}
