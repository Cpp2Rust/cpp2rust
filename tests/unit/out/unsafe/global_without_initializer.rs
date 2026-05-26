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
pub static mut s_s: *mut S = unsafe { std::ptr::null_mut() };
pub static mut s_file: *mut ::libc::FILE = unsafe { std::ptr::null_mut() };
pub static mut s_size: u64 = unsafe { 0_u64 };
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((s_s).is_null());
    assert!((s_file).is_null());
    assert!(((s_size) == (0_u64)));
    return 0;
}
