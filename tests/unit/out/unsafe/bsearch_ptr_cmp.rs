extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn int_cmp_0(mut v1: *const ::libc::c_void, mut v2: *const ::libc::c_void) -> i32 {
    return ((*(v1 as *const i32)) - (*(v2 as *const i32)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a1: [i32; 3] = [1, 2, 3];
    let mut vptr1: *mut ::libc::c_void = libc::bsearch(
        ((&mut a1[(0) as usize] as *mut i32) as *const i32 as *const ::libc::c_void),
        (a1.as_mut_ptr() as *const i32 as *const ::libc::c_void),
        1_usize,
        ::std::mem::size_of::<i32>(),
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(
            (int_cmp_0 as unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32)
                as *const (),
        )),
    );
    assert!(((vptr1) == ((&mut a1[(0) as usize] as *mut i32) as *mut i32 as *mut ::libc::c_void)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
