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
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 1;
    let mut p: *mut i32 = (&mut a as *mut i32);
    (unsafe { libc::printf(c"%p %d".as_ptr() as *const libc::c_char, (p), (*p)) });
    let mut q: *mut ::libc::c_void = ((&mut a as *mut i32) as *mut ::libc::c_void);
    (unsafe { libc::printf(c" %p\n".as_ptr() as *const libc::c_char, (q)) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
