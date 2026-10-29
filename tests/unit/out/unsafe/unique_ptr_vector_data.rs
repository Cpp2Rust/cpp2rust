extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn first_0(u: *const Option<Box<Vec<u8>>>) -> *mut u8 {
    return (*(*u).as_deref().map_or(::std::ptr::null_mut(), |v| {
        v as *const Vec<u8> as *mut Vec<u8>
    }))
    .as_mut_ptr();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Option<Box<Vec<u8>>> = Some(Box::from_raw(
        (Box::leak(Box::new(
            (0..(3_usize) as usize).map(|_| 0_u8).collect::<Vec<_>>(),
        )) as *mut Vec<u8>),
    ));
    let mut p: *mut u8 = (*v.as_deref().map_or(::std::ptr::null_mut(), |v| {
        v as *const Vec<u8> as *mut Vec<u8>
    }))
    .as_mut_ptr();
    (*p.offset((2) as isize)) = 1_u8;
    (*(unsafe { first_0(&v) }).offset((0) as isize)) = 5_u8;
    assert!((((*p.offset((0) as isize)) as i32) == (5)));
    assert!((((*(unsafe { first_0(&v,) }).offset((2) as isize)) as i32) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
