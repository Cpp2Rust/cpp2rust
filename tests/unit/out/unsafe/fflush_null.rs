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
    let mut file_ptr: *mut ::std::fs::File = std::ptr::null_mut();
    return if !(file_ptr).is_null() {
        match (*file_ptr).sync_all() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        ::std::io::stdout().flush().unwrap();
        ::std::io::stderr().flush().unwrap();
        0
    };
}
