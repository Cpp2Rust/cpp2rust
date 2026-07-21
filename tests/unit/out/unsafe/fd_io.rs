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
        (c"/tmp/cpp2rust_fd_io_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fd: i32 = (unsafe {
        libc::open(
            path as *const i8,
            (((::libc::O_WRONLY) | (::libc::O_CREAT)) | (::libc::O_TRUNC)) as i32,
            (420),
        )
    });
    assert!(((((fd) >= (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fd,
            (c"hello world".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            11_usize
        )) == (11_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    fd = (unsafe { libc::open(path as *const i8, ::libc::O_RDONLY as i32) });
    assert!(((((fd) >= (0)) as i32) != 0));
    let mut buf: [libc::c_char; 16] = [(0 as libc::c_char); 16];
    {
        let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[libc::c_char; 16]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 16]>()
        )) == (11_isize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"hello world".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::read(
            fd,
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 16]>()
        )) == (0_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fd)) == (0)) as i32) != 0));
    assert!(((((libc::unlink(path)) == (0)) as i32) != 0));
    return 0;
}
