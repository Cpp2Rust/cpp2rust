extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut b1: u8 = 1_u8;
    let mut ushift1: u32 = 3_u32;
    let mut shl1: u8 = ((b1 as u32) << ushift1) as u8;
    assert!((shl1 == ((8) as u8)));
    let mut ushift2: u32 = 2_u32;
    let mut shr1: u8 = ((shl1 as u32) >> ushift2) as u8;
    assert!((shr1 == ((2) as u8)));
    let mut ushift3: u32 = 5_u32;
    {
        let __n = ((b1 as u32) << ushift3) as u8;
        b1 = __n;
        b1
    };
    assert!((b1 == ((32) as u8)));
    let mut ushift4: u32 = 3_u32;
    {
        let __n = ((b1 as u32) >> ushift4) as u8;
        b1 = __n;
        b1
    };
    assert!((b1 == ((4) as u8)));
    let mut ishift: i32 = 1;
    let mut shl2: u8 = ((b1 as u32) << ishift) as u8;
    assert!((shl2 == ((8) as u8)));
    let mut b2: u8 = 129_u8;
    let mut ushift5: u32 = 8_u32;
    let mut shl3: u8 = ((b2 as u32) << ushift5) as u8;
    assert!((shl3 == ((0) as u8)));
    let mut ushift6: u32 = 9_u32;
    let mut shr2: u8 = ((b2 as u32) >> ushift6) as u8;
    assert!((shr2 == ((0) as u8)));
    let mut ushift7: u32 = 9_u32;
    {
        let __n = ((b2 as u32) << ushift7) as u8;
        b2 = __n;
        b2
    };
    assert!((b2 == ((0) as u8)));
    let mut b3: u8 = 129_u8;
    let mut ushift8: u32 = 8_u32;
    {
        let __n = ((b3 as u32) >> ushift8) as u8;
        b3 = __n;
        b3
    };
    assert!((b3 == ((0) as u8)));
    let mut b4: u8 = 255_u8;
    let mut ishift2: i32 = 12;
    let mut shl4: u8 = ((b4 as u32) << ishift2) as u8;
    assert!((shl4 == ((0) as u8)));
    let mut lshift: u64 = 10_u64;
    let mut shl5: u8 = ((b4 as u32) << lshift) as u8;
    assert!((shl5 == ((0) as u8)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
