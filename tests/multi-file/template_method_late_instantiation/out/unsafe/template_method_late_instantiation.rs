extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S_int_ {
    pub x: i32,
}
impl S_int_ {
    pub unsafe fn set(&mut self, mut v: i32) {
        self.x = v;
    }
}
impl Default for S_int_ {
    fn default() -> Self {
        S_int_ { x: 0 }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: S_int_ = <S_int_>::default();
    (unsafe { S_int_::set(&mut p, 3) });
    assert!(((unsafe { f_0((&mut p as *mut S_int_),) }) == (3)));
    return 0;
}
impl S_int_ {
    pub unsafe fn get(&mut self) -> i32 {
        return self.x;
    }
}
pub unsafe fn f_0(mut p: *mut S_int_) -> i32 {
    return (unsafe { S_int_::get(&mut (*p)) });
}
