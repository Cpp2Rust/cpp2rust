extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: Point = <Point>::default();
    p.x = 67305985;
    p.y = 134678021;
    let mut bytes: *mut u8 = ((&mut p as *mut Point) as *mut u8);
    assert!((((*bytes.offset((0) as isize)) as i32) == (1)));
    assert!((((*bytes.offset((3) as isize)) as i32) == (4)));
    assert!((((*bytes.offset((4) as isize)) as i32) == (5)));
    assert!((((*bytes.offset((7) as isize)) as i32) == (8)));
    return 0;
}
