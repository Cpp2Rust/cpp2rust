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
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Box_int_ {
    pub val: i32,
}
impl Box_int_ {
    pub unsafe fn twice(&mut self) -> i32 {
        return ((self.val) + (self.val));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Box_Point_ {
    pub val: Point,
}
impl Box_Point_ {
    pub unsafe fn get(&mut self) -> Point {
        return self.val.clone();
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i: Box_int_ = Box_int_ { val: 3 };
    assert!(((unsafe { i.twice() }) == (6)));
    let mut p: Box_Point_ = Box_Point_ {
        val: Point { x: 4 },
    };
    assert!((((unsafe { p.get() }).x) == (4)));
    return 0;
}
