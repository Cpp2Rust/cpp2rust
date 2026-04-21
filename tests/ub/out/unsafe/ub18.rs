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
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut p: *mut i32 = arr.as_mut_ptr();

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        p,
        libcc2rs::malloc_usable_size(p as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    return 0;
}
