extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Pair {
    pub first: u32,
    pub second: u32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pt: Point = Point {
        x: 10_u32,
        y: 20_u32,
    };
    let mut pair: *mut Pair = ((&mut pt as *mut Point) as *mut Pair);
    assert!((((*pair).first) == (10_u32)));
    assert!((((*pair).second) == (20_u32)));
    (*pair).first = 42_u32;
    assert!(((pt.x) == (42_u32)));
    return 0;
}
