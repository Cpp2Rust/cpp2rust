extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct point {
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut src: point = point { x: 3, y: 7 };
    let mut buf: [u8; 8] = [0_u8; 8];
    {
        if ::std::mem::size_of::<[u8; 8]>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((&mut src as *mut point) as *const point as *const ::libc::c_void),
                (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
                ::std::mem::size_of::<[u8; 8]>() as usize,
            )
        }
        (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
    };
    let mut dst: point = <point>::default();
    {
        if ::std::mem::size_of::<point>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                (buf.as_mut_ptr() as *const u8 as *const ::libc::c_void),
                ((&mut dst as *mut point) as *mut point as *mut ::libc::c_void),
                ::std::mem::size_of::<point>() as usize,
            )
        }
        ((&mut dst as *mut point) as *mut point as *mut ::libc::c_void)
    };
    assert!(((((dst.x) == (3)) as i32) != 0));
    assert!(((((dst.y) == (7)) as i32) != 0));
    return 0;
}
