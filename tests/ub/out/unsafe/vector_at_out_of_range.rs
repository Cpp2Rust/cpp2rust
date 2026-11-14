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
    let mut v: Vec<i32> = Vec::from([1, 2, 3]);
    let mut idx: usize = v.len();
    let mut p: *mut i32 = (&mut (v)[idx] as *mut i32);
    return (((p).is_null()) as i32);
}
pub unsafe fn __cpp2rust_init_globals() {}
