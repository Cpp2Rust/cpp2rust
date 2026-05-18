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
    let mut b: i32 = 2;
    let mut b_ptr: *mut i32 = (&mut b as *mut i32);
    (*b_ptr) = 3;
    let mut b_ptr_ptr: *mut *mut i32 = (&mut b_ptr as *mut *mut i32);
    (*(*b_ptr_ptr)) = 4;
    (*b_ptr) = (*(*b_ptr_ptr));
    let mut offset: u64 =
        ((((b_ptr as usize - b_ptr as usize) / ::std::mem::size_of::<i32>()) as i64) as u64);
    return b;
}
