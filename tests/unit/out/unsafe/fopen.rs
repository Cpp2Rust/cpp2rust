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
    let mut fname: *const u8 = b"testfile.txt\0".as_ptr();
    let mut mode: *const u8 = b"rb\0".as_ptr();
    let mut file_ptr: *mut ::std::fs::File = match std::ffi::CStr::from_ptr(mode as *const i8)
        .to_str()
        .expect("invalid c-string")
    {
        v if v == "rb" => std::fs::OpenOptions::new()
            .read(true)
            .open(
                std::ffi::CStr::from_ptr(fname as *const i8)
                    .to_str()
                    .expect("invalid c-string"),
            )
            .ok()
            .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
        v if v == "wb" => std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(
                std::ffi::CStr::from_ptr(fname as *const i8)
                    .to_str()
                    .expect("invalid c-string"),
            )
            .ok()
            .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
        _ => panic!("unsupported mode"),
    };
    return 0;
}
