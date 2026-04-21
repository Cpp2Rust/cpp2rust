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
    let mut arr1: [i32; 2] = [1, 2];
    arr1[(0) as usize] = 3;
    arr1[(1) as usize] = 4;
    return ((arr1[(0) as usize]) + (arr1[(1) as usize]));
}
