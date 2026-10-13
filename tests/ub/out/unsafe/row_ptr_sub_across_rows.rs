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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut rows: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    return ((((rows[(1) as usize].as_mut_ptr() as usize
        - ((&mut rows[(0) as usize][(0) as usize] as *mut i32).offset((2) as isize)) as usize)
        / ::std::mem::size_of::<i32>()) as i64) as i32);
}
pub unsafe fn __cpp2rust_init_globals() {}
