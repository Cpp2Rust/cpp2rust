extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn write_ulong_0(p: *mut u64) {
    (*p) = 42_u64;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: usize = 0_usize;
    (unsafe { write_ulong_0((&mut x as *mut usize as *mut u64)) });
    assert!(((((x) == (42_usize)) as i32) != 0));
    return 0;
}
