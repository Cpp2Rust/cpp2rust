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
    let a: i32 = 1;
    let mut p: *const i32 = (&a as *const i32);
    let mut q: *mut i32 = (((((p) as *const i32 as *const ::libc::c_void) as u64)
        as *mut ::libc::c_void) as *mut i32);
    assert!(((p) == ((q).cast_const())));
    return 0;
}
