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
    let mut x: u32 = (-1_i32 as u32);
    assert!(((x) == (<u32>::MAX)));
    let mut v1: i32 = (((x) & (1_u32)) as i32);
    let mut v2: u32 = ((x) & (1_u32));
    let mut p: *mut u32 = (&mut x as *mut u32);
    let mut b: bool = (((*p) & (255_u32)) != 0);
    let mut a: i32 = (((*p) & (255_u32)) as i32);
    return a;
}
