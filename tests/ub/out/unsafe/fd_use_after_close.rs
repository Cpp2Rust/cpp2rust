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
    let mut fd: i32 = (unsafe {
        libc::open(
            (c"/dev/null".as_ptr().cast_mut()).cast_const() as *const i8,
            0 as i32,
        )
    });
    libc::close(fd);
    let mut buf: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    let mut n: isize = libc::read(
        fd,
        (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
        ::std::mem::size_of::<[libc::c_char; 4]>(),
    );
    return if ((((n) == (-1_i32 as isize)) as i32) != 0) {
        0
    } else {
        1
    };
}
