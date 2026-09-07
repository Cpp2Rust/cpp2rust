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
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fds[(1) as usize],
            (c"ab".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            2_usize
        )) == (2_isize)) as i32)
            != 0)
    );
    let mut buf: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    {
        let byte_0 = (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[libc::c_char; 4]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 4]>()
        )) == (2_isize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"ab".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
    assert!(
        ((((libc::read(
            fds[(0) as usize],
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            ::std::mem::size_of::<[libc::c_char; 4]>()
        )) == (0_isize)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
    return 0;
}
