extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fallthrough_into_block_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 => {
            r += 1;
        }
        v if v == 2 => {
            let mut tmp: i32 = ((r) * (10));
            r = ((tmp) + (5));
            break;
        }
        _ => {
            r = -1_i32;
            break;
        }
    });
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
            fallthrough_into_block_0(_x)
        }) == (15))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            fallthrough_into_block_0(_x)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            fallthrough_into_block_0(_x)
        }) == (-1_i32))
    );
    return 0;
}
