extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn stacked_block_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 || v == 2 || v == 3 => {
            let mut y: i32 = ((x) * (2));
            r = ((y) + (1));
            break;
        }
        _ => {
            r = 0;
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
            let _x: i32 = 2;
            stacked_block_0(_x)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            stacked_block_0(_x)
        }) == (0))
    );
    return 0;
}
