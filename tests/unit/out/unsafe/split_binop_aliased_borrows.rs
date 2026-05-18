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
    let mut v: Vec<i32> = vec![1, 2];
    let mut p: *mut i32 = v.as_mut_ptr();
    let r: *const i32 = &v[(1_u64) as usize] as *const i32;
    (*p) = (*r);
    return v[(0_u64) as usize];
}
