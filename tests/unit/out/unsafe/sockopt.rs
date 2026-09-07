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
    let mut on: i32 = 1;
    assert!(
        ((((libc::setsockopt(
            s,
            libc::SOL_SOCKET,
            libc::SO_KEEPALIVE,
            ((&mut on as *mut i32) as *const i32 as *const ::libc::c_void),
            (::std::mem::size_of::<i32>() as u32)
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::setsockopt(
            s,
            libc::IPPROTO_TCP,
            libc::TCP_NODELAY,
            ((&mut on as *mut i32) as *const i32 as *const ::libc::c_void),
            (::std::mem::size_of::<i32>() as u32)
        )) == (0)) as i32)
            != 0)
    );
    let mut err: i32 = -1_i32;
    let mut len: u32 = (::std::mem::size_of::<i32>() as u32);
    assert!(
        ((((libc::getsockopt(
            s,
            libc::SOL_SOCKET,
            libc::SO_ERROR,
            ((&mut err as *mut i32) as *mut i32 as *mut ::libc::c_void),
            (&mut len as *mut u32)
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((err) == (0)) as i32) != 0));
    assert!(((((libc::close(s)) == (0)) as i32) != 0));
    return 0;
}
