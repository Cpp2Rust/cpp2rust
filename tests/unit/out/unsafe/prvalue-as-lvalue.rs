extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(a: *const i32) -> *const i32 {
    return a;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 1;
    let mut pa: *mut i32 = (&mut a as *mut i32);
    let b: *const i32 = (unsafe {
        let _a: *const i32 = &(*pa) as *const i32;
        foo_0(_a)
    });
    return (*b);
}
