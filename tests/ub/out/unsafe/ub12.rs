extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn escape_0(mut ptr: *mut i32) {
    ::std::mem::drop(Box::from_raw(ptr));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut alloc: *mut i32 = (Box::leak(Box::new(1)) as *mut i32);
    (unsafe {
        let _ptr: *mut i32 = alloc;
        escape_0(_ptr)
    });
    ::std::mem::drop(Box::from_raw(alloc));
    return 0;
}
