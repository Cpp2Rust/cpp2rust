extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut a0: UnsafeRandomAccessIterator<i32>) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vec_: Vec<i32> = Vec::from_fn(4_usize, |_| 0_i32);
    let mut it: UnsafeRandomAccessIterator<i32> = vec_.begin();
    (unsafe { foo_0((it).clone()) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
