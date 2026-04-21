extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut Pair = (Box::leak(Box::new(Pair { x: 1, y: 2 })) as *mut Pair);
    let mut out: i32 = (((*p).x) + ((*p).y));
    ::std::mem::drop(Box::from_raw(p));
    return out;
}
