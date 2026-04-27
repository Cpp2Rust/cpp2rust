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
pub struct XX {
    pub x: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut obj: XX = <XX>::default();
    let mut ptr: *mut XX = (&mut obj as *mut XX);
    (*ptr).x = 2;
    let mut c: bool = false;
    let mut r: i32 = if c { obj.x } else { (*ptr).x };
    let mut p: *mut i32 = (&mut obj.x as *mut i32);
    return ((*p) + (r));
}
