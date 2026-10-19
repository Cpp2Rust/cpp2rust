extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn second_row_0() -> *mut i32 {
    let mut rows: Vec<Vec<i32>> = Vec::from([Vec::from([1, 2, 3]), Vec::from([4, 5, 6])]);
    return rows[(1_usize)].as_mut_ptr();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut i32 = (unsafe { second_row_0() });
    return (*p.offset((0) as isize));
}
pub unsafe fn __cpp2rust_init_globals() {}
