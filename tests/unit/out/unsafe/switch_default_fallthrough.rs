extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn default_chain_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 2 => {
            r = 2;
        }
        _ => {
            r = (((r) * (10)) + (9));
        }
        __v if __v == 1 => {
            r = (((r) * (10)) + (1));
        }
    });
    return r;
}
pub unsafe fn default_break_1(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        _ => {
            r = 9;
            break;
        }
        __v if __v == 1 => {
            r = 1;
            break;
        }
        __v if __v == 2 => {
            r = 2;
        }
    });
    return r;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { default_chain_0(99,) }) == (91)));
    assert!(((unsafe { default_chain_0(2,) }) == (291)));
    assert!(((unsafe { default_chain_0(1,) }) == (1)));
    assert!(((unsafe { default_break_1(2,) }) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
