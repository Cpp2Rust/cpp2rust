extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut b1: u8 = 1_u8;
    let mut ushift1: u32 = 3_u32;
    let mut shl1: u8 = b1 << ushift1;
    assert!(((shl1) == (8)));
    let mut ushift2: u32 = 2_u32;
    let mut shr1: u8 = shl1 >> ushift2;
    assert!(((shr1) == (2)));
    let mut ushift3: u32 = 5_u32;
    {
        let n_ = b1 << ushift3;
        b1 = n_;
        b1
    };
    assert!(((b1) == (32)));
    let mut ushift4: u32 = 3_u32;
    {
        let n_ = b1 >> ushift4;
        b1 = n_;
        b1
    };
    assert!(((b1) == (4)));
    return 0;
}
