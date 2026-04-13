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
    let mut arr: *mut i32 =
        Box::leak((0..15_u64).map(|_| 0 as i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut ptr: *mut i32 = arr.offset((15) as isize);
    let mut out: i32 = (*ptr);

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        arr,
        libcc2rs::malloc_usable_size(arr as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    return out;
}
