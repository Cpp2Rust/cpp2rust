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
    let mut pw: *mut passwd = libc::getpwuid(libc::geteuid());
    if !!(pw).is_null() {
        return 0;
    }
    let mut home: *mut ::core::ffi::c_char = (*pw).pw_dir;
    let mut d: *mut dirent =
        libc::readdir(libc::opendir((c"/tmp".as_ptr().cast_mut()).cast_const()));
    let mut dname: *mut ::core::ffi::c_char = (*d).d_name.as_mut_ptr();
    return 0;
}
