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
    let mut path: *const libc::c_char =
        (c"/tmp/cpp2rust_termios_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: i32 = (unsafe {
        libc::open(
            path as *const i8,
            (((::libc::O_RDWR) | (::libc::O_CREAT)) | (::libc::O_TRUNC)) as i32,
            (420),
        )
    });
    assert!(((((fd) >= (0)) as i32) != 0));
    let mut tio: ::libc::termios = unsafe { std::mem::zeroed() };
    assert!(
        ((((libc::tcgetattr(fd, (&mut tio as *mut ::libc::termios))) == (-1_i32)) as i32) != 0)
    );
    assert!(
        ((((libc::tcsetattr(fd, 0, (&mut tio as *mut ::libc::termios).cast_const())) == (-1_i32))
            as i32)
            != 0)
    );
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
    return 0;
}
