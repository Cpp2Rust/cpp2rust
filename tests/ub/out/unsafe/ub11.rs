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
    let mut element: *mut i32 = (Box::leak(Box::new(10)) as *mut i32);
    let mut ptr: *mut i32 = element.offset((1) as isize);
    let mut out: i32 = (*ptr);
    {
        let __p = element;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return out;
}
pub unsafe fn __cpp2rust_init_globals() {}
