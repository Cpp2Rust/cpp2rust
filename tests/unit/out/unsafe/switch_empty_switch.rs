extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn empty_switch_0(mut x: i32) -> i32 {
    'switch: {
        let __match_cond = x;
        match __match_cond {
            _ => {}
        }
    };
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            empty_switch_0(_x)
        }) == (5))
    );
    return 0;
}
