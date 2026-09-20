extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0(mut p: *mut i32, mut n: i32) -> i32 {
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        total += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return total;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut array: *mut i32 =
        Box::leak((0..100_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();

    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        array,
        libcc2rs::malloc_usable_size(array as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    let mut filled: *mut i32 =
        Box::leak((0..4_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (4)) {
        (*filled.offset((i) as isize)) = ((i) + (1));
        i.prefix_inc();
    }
    if ((unsafe {
        let _p: *mut i32 = filled;
        sum_0(_p, 4)
    }) != (10))
    {
        return 1;
    }
    ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
        filled,
        libcc2rs::malloc_usable_size(filled as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
    )));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
