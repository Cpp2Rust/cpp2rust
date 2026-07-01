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
    let mut a: u32 = 42_u32;
    assert!(
        ((((*(((&mut a as *mut u32) as *mut ::libc::c_void) as *mut i32)) == (42)) as i32) != 0)
    );
    return 0;
}
