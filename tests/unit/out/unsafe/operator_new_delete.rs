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
    let mut a: *mut i32 = (libcc2rs::malloc_unsafe(::std::mem::size_of::<i32>()) as *mut i32);
    (*a) = 42;
    assert!(((*a) == (42)));
    libcc2rs::free_unsafe((a as *mut i32 as *mut ::libc::c_void));
    let mut arr: *mut i32 =
        (libcc2rs::malloc_unsafe((::std::mem::size_of::<i32>() as usize).wrapping_mul(2_usize))
            as *mut i32);
    (*arr.offset((0) as isize)) = 0;
    (*arr.offset((1) as isize)) = 1;
    assert!((((*arr.offset((0) as isize)) + (*arr.offset((1) as isize))) == (1)));
    libcc2rs::free_unsafe((arr as *mut i32 as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
