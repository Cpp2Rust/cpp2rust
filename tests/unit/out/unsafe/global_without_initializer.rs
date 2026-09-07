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
pub struct S {
    pub a: i32,
}
pub static mut s_0: *mut S = unsafe { std::ptr::null_mut() };
pub static mut file_1: *mut ::libc::FILE = unsafe { std::ptr::null_mut() };
pub static mut size_2: usize = unsafe { 0_usize };
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((s_0).is_null());
    assert!((file_1).is_null());
    assert!(((size_2) == (0_usize)));
    return 0;
}
