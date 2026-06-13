extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn log_0(
    mut file: *const core::ffi::c_char,
    mut line: i32,
    mut func: *const core::ffi::c_char,
) {
    printf(c"%s %d %s\n".as_ptr() as *const i8, file, line, func);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    printf(
        c"%s %d %s\n".as_ptr() as *const i8,
        c"macros.cpp".as_ptr(),
        8,
        c"main".as_ptr(),
    );
    (unsafe {
        let _func: *const core::ffi::c_char = c"main".as_ptr();
        log_0(c"macros.cpp".as_ptr(), 9, _func)
    });
    return 0;
}
