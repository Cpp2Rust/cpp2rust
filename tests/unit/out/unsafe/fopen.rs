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
    let mut fname: *const libc::c_char = c"testfile.txt".as_ptr();
    let mut mode: *const libc::c_char = c"rb".as_ptr();
    let mut file_ptr: *mut ::libc::FILE = libc::fopen(fname, mode);
    return 0;
}
