extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut array: *mut i32 =
        Box::leak((0..100_u64).map(|_| 0 as i32).collect::<Box<[i32]>>()).as_mut_ptr();
    {
        let byte_0 = (array as *mut i32 as *mut ::libc::c_void) as *mut u8;
        for offset in 0..(::std::mem::size_of::<i32>() as u64 as u64).wrapping_mul(100_u64) {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (array as *mut i32 as *mut ::libc::c_void)
    };
    (*array.offset((99) as isize)) = -1_i32;
    let mut p1: *mut i32 = array;
    'loop_: while ((*p1) >= (0)) {
        (*p1) = 1;
        p1.prefix_inc();
    }
    let mut out: i32 = 0;
    let mut p1: *mut i32 = array;
    'loop_: while ((*p1) >= (0)) {
        out += (*p1);
        p1.prefix_inc();
    }
    let mut p2: *mut i32 = array;

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        p2,
        libcc2rs::malloc_usable_size(p2 as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    return out;
}
