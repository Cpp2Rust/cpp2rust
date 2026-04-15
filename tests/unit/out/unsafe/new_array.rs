extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut array: *mut i32 =
        Box::leak((0..100_u64).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        array,
        libcc2rs::malloc_usable_size(array as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    return 0;
}
