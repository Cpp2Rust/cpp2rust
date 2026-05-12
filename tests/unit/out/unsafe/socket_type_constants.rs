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
    assert!(((((libc::SOCK_STREAM) == (1)) as i32) != 0));
    assert!(((((libc::SOCK_DGRAM) == (2)) as i32) != 0));
    assert!(((((libc::SOCK_RAW) == (3)) as i32) != 0));
    assert!(((((libc::SOCK_RDM) == (4)) as i32) != 0));
    assert!(((((libc::SOCK_SEQPACKET) == (5)) as i32) != 0));
    assert!(((((libc::SOCK_DCCP) == (6)) as i32) != 0));
    assert!(((((libc::SOCK_PACKET) == (10)) as i32) != 0));
    let mut x: i32 = ((libc::SOCK_STREAM) | (libc::SOCK_CLOEXEC));
    assert!((((((x) & (libc::SOCK_STREAM)) == (libc::SOCK_STREAM)) as i32) != 0));
    assert!((((((x) & (libc::SOCK_CLOEXEC)) == (libc::SOCK_CLOEXEC)) as i32) != 0));
    let mut y: i32 = ((libc::SOCK_DGRAM) | (libc::SOCK_NONBLOCK));
    assert!((((((y) & (libc::SOCK_DGRAM)) == (libc::SOCK_DGRAM)) as i32) != 0));
    assert!((((((y) & (libc::SOCK_NONBLOCK)) == (libc::SOCK_NONBLOCK)) as i32) != 0));
    return 0;
}
