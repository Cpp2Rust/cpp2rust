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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 42;
    let mut ptr1: *mut i32 = (&mut i1 as *mut i32);
    let mut ptr2: *mut libc::c_char = ((&mut i1 as *mut i32) as *mut libc::c_char);
    let mut vptr1: *mut ::libc::c_void = (ptr1 as *mut i32 as *mut ::libc::c_void);
    let mut vptr2: *mut ::libc::c_void = (ptr2 as *mut libc::c_char as *mut ::libc::c_void);
    assert!(((vptr1) == (vptr2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
