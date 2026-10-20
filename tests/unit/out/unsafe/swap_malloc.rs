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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: *mut i32 =
        (libcc2rs::malloc_unsafe((3_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)))
            as *mut i32);
    (*buf.offset((0) as isize)) = 1;
    (*buf.offset((1) as isize)) = 2;
    (*buf.offset((2) as isize)) = 3;
    cpp_swap_unsafe!(buf.offset((0) as isize), buf.offset((2) as isize));
    assert!(
        (((*buf.offset((0) as isize)) == (3)) && ((*buf.offset((1) as isize)) == (2)))
            && ((*buf.offset((2) as isize)) == (1))
    );
    cpp_swap_unsafe!(buf.offset((1) as isize), buf.offset((1) as isize));
    assert!(((*buf.offset((1) as isize)) == (2)));
    let mut local: i32 = 4;
    cpp_swap_unsafe!(buf.offset((1) as isize), &mut local);
    assert!(((*buf.offset((1) as isize)) == (4)) && ((local) == (2)));
    let mut other: *mut i32 = (libcc2rs::malloc_unsafe(::std::mem::size_of::<i32>()) as *mut i32);
    (*other) = 5;
    cpp_swap_unsafe!(buf, other);
    assert!(((*buf.offset((0) as isize)) == (5)) && ((*other) == (3)));
    let mut bytes: *mut u8 = (buf as *mut u8);
    (*bytes.offset((0) as isize)) = 170_u8;
    (*bytes.offset((4) as isize)) = 187_u8;
    cpp_swap_unsafe!(bytes.offset((0) as isize), bytes.offset((4) as isize));
    assert!(
        (((*bytes.offset((0) as isize)) as i32) == (187))
            && (((*bytes.offset((4) as isize)) as i32) == (170))
    );
    let mut words: [u32; 2] = [16909060_u32, 84281096_u32];
    let mut halves: *mut u16 = (words.as_mut_ptr() as *mut u16);
    cpp_swap_unsafe!(halves.offset((0) as isize), halves.offset((3) as isize));
    assert!(((words[(0) as usize]) == (16909574_u32)) && ((words[(1) as usize]) == (50595592_u32)));
    libcc2rs::free_unsafe((other as *mut ::libc::c_void));
    libcc2rs::free_unsafe((buf as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
