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
    let mut fd: i32 = 0;
    let mut ssloc: sockaddr_storage = unsafe { std::mem::zeroed::<sockaddr_storage>() };
    let mut slen: u32 = (::std::mem::size_of::<sockaddr_storage>() as u32);
    assert!(
        ((((libc::getsockname(
            fd,
            ((&mut ssloc as *mut sockaddr_storage) as *mut sockaddr),
            (&mut slen as *mut u32)
        )) == (-1_i32)) as i32)
            != 0)
    );
    let mut sin: sockaddr_in = unsafe { std::mem::zeroed::<sockaddr_in>() };
    let mut inlen: u32 = (::std::mem::size_of::<sockaddr_in>() as u32);
    assert!(
        ((((libc::getsockname(
            fd,
            ((&mut sin as *mut sockaddr_in) as *mut sockaddr),
            (&mut inlen as *mut u32)
        )) == (-1_i32)) as i32)
            != 0)
    );
    return 0;
}
