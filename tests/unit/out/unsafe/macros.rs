extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn log_0(mut file: *const u8, mut line: i32, mut func: *const u8) {
    printf(b"%s %d %s\n\0".as_ptr() as *const i8, file, line, func);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    printf(
        b"%s %d %s\n\0".as_ptr() as *const i8,
        b"macros.cpp\0".as_ptr(),
        8,
        b"main\0".as_ptr(),
    );
    (unsafe {
        let _file: *const u8 = b"macros.cpp\0".as_ptr();
        let _line: i32 = 9;
        let _func: *const u8 = b"main\0".as_ptr();
        log_0(_file, _line, _func)
    });
    return 0;
}
