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
    let mut p: Option<Box<i32>> = Some(Box::new(10));
    (*p.as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)) += 5;
    (*p.as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)) -= 3;
    (*p.as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)) *= 2;
    let mut q: Option<Box<i32>> = Some(Box::new(1));
    let mut sum: i32 = ((*p
        .as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
        + (*q
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)));
    assert!(((sum) == (25)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
