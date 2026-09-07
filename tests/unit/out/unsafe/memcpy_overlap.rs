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
    let mut buf: [libc::c_char; 6] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
        (5 as libc::c_char),
        (6 as libc::c_char),
    ];
    {
        if 4_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void),
                (buf.as_mut_ptr().offset((2) as isize) as *mut libc::c_char as *mut ::libc::c_void),
                4_usize as usize,
            )
        }
        (buf.as_mut_ptr().offset((2) as isize) as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(((buf[(0) as usize] as i32) == (1)));
    assert!(((buf[(1) as usize] as i32) == (2)));
    assert!(((buf[(2) as usize] as i32) == (1)));
    assert!(((buf[(3) as usize] as i32) == (2)));
    assert!(((buf[(4) as usize] as i32) == (3)));
    assert!(((buf[(5) as usize] as i32) == (4)));
    return 0;
}
