extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut single: *mut i32) {
    ::std::mem::drop(Box::from_raw(single));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: *mut i32 =
        Box::leak((0..10_u64).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    (unsafe {
        let _single: *mut i32 = x;
        foo_0(_single)
    });
    return 0;
}
