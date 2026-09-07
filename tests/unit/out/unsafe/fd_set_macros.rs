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
    let mut set: ::libc::fd_set = std::mem::zeroed::<::libc::fd_set>();
    libc::FD_ZERO((&mut set as *mut ::libc::fd_set));
    assert!(
        ((!(libc::FD_ISSET(3, (&mut set as *mut ::libc::fd_set).cast_const()) as i32 != 0) as i32)
            != 0)
    );
    libc::FD_SET(3, (&mut set as *mut ::libc::fd_set));
    assert!((libc::FD_ISSET(3, (&mut set as *mut ::libc::fd_set).cast_const()) as i32 != 0));
    libc::FD_CLR(3, (&mut set as *mut ::libc::fd_set));
    assert!(
        ((!(libc::FD_ISSET(3, (&mut set as *mut ::libc::fd_set).cast_const()) as i32 != 0) as i32)
            != 0)
    );
    return 0;
}
