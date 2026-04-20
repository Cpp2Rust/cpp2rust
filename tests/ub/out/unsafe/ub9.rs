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
    let mut arr: *mut i32 =
        Box::leak((0..10_u64).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut out: i32 = (*arr.offset((10) as isize));

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        arr,
        libcc2rs::malloc_usable_size(arr as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    return out;
}
