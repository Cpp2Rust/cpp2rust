extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut array: *mut i32) {
    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        array,
        libcc2rs::malloc_usable_size(array as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: *mut i32 = (Box::leak(Box::new(1)) as *mut i32);
    (unsafe {
        let _array: *mut i32 = x;
        foo_0(_array)
    });
    return 0;
}
