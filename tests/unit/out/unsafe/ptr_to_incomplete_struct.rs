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
    let mut fp: *mut ::libc::FILE = libcc2rs::stdout_unsafe();
    let mut p: *mut ::libc::c_void = (fp as *mut ::libc::c_void);
    let mut fp2: *mut ::libc::FILE = (p as *mut ::libc::FILE);
    assert!(((((fp) == (fp2)) as i32) != 0));
    return 0;
}
