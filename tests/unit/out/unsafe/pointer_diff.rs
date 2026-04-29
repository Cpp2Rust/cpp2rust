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
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut p0: *const i32 = (&mut a[(0) as usize] as *mut i32).cast_const();
    let mut p1: *const i32 = (&mut a[(4) as usize] as *mut i32).cast_const();
    return ((((p1 as usize - p0 as usize) / ::std::mem::size_of::<i32>()) as u64) as i32);
}
