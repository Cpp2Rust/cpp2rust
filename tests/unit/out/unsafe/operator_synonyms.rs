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
    let mut a: u32 = 12_u32;
    let mut b: u32 = 10_u32;
    let mut z: u32 = 0_u32;
    assert!((!(z != 0)));
    assert!(((a != 0) && (b != 0)));
    assert!((!((a != 0) && (z != 0))));
    assert!(((a != 0) || (z != 0)));
    assert!(((a) != (b)));
    assert!(((!a) == (!12_u32)));
    assert!((((a) & (b)) == (8_u32)));
    assert!((((a) | (b)) == (14_u32)));
    assert!((((a) ^ (b)) == (6_u32)));
    a &= b;
    assert!(((a) == (8_u32)));
    a |= b;
    assert!(((a) == (10_u32)));
    a ^= b;
    assert!(((a) == (0_u32)));
    return 0;
}
