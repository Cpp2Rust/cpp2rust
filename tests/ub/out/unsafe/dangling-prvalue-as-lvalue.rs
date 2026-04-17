extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
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
    let mut v: Vec<i32> = vec![1, 2];
    let b: *const i32 = (unsafe {
        let _a: *const i32 = &(*v.as_mut_ptr()) as *const i32;
        foo_0(_a)
    });
    v.clear();
    return (*b);
}
