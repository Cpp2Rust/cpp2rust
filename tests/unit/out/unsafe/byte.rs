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
    let mut ishift1: i32 = 3;
    let mut shl1: u8 = b1 << ishift1;
    assert!(((shl1) == (8)));
    let mut ishift2: i32 = 2;
    let mut shr1: u8 = shl1 >> ishift2;
    assert!(((shr1) == (2)));
    let mut ishift3: i32 = 5;
    {
        let n_ = b1 << ishift3;
        b1 = n_;
        b1
    };
    assert!(((b1) == (32)));
    let mut ishift4: i32 = 3;
    {
        let n_ = b1 >> ishift4;
        b1 = n_;
        b1
    };
    assert!(((b1) == (4)));
    let mut b2: u8 = 1_u8;
    let mut ushift1: u32 = 3_u32;
    let mut shl2: u8 = b2 << ushift1;
    assert!(((shl2) == (8)));
    let mut ushift2: u32 = 2_u32;
    let mut shr2: u8 = shl2 >> ushift2;
    assert!(((shr2) == (2)));
    let mut ushift3: u32 = 5_u32;
    {
        let n_ = b2 << ushift3;
        b2 = n_;
        b2
    };
    assert!(((b2) == (32)));
    let mut ushift4: u32 = 3_u32;
    {
        let n_ = b2 >> ushift4;
        b2 = n_;
        b2
    };
    assert!(((b2) == (4)));
    return 0;
}
