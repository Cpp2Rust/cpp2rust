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
    let b1: Value<u8> = Rc::new(RefCell::new(1_u8));
    let mut ushift1: u32 = 3_u32;
    let mut shl1: u8 = (*b1.borrow()) << ushift1;
    assert!((shl1 == ((8) as u8)));
    let mut ushift2: u32 = 2_u32;
    let mut shr1: u8 = shl1 >> ushift2;
    assert!((shr1 == ((2) as u8)));
    let mut ushift3: u32 = 5_u32;
    {
        let n_ = (*b1.borrow()) << ushift3;
        (*b1.borrow_mut()) = n_;
        (*b1.borrow())
    };
    assert!(((*b1.borrow()) == ((32) as u8)));
    let mut ushift4: u32 = 3_u32;
    {
        let n_ = (*b1.borrow()) >> ushift4;
        (*b1.borrow_mut()) = n_;
        (*b1.borrow())
    };
    assert!(((*b1.borrow()) == ((4) as u8)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
