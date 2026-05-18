extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut a0: *mut i32) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vec_: Vec<i32> = (0..(4_u64) as usize)
        .map(|_| <i32>::default())
        .collect::<Vec<_>>();
    let mut it: *mut i32 = vec_.as_mut_ptr();
    (unsafe {
        let _a0: *mut i32 = it;
        foo_0(_a0)
    });
    return 0;
}
