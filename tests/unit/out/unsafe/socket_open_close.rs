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
    let mut s: i32 = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    assert!(((((s) >= (0)) as i32) != 0));
    assert!(((((libc::close(s)) == (0)) as i32) != 0));
    return 0;
}
