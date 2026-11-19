extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn set_0(r: *mut i32) {
    (*r) = 7;
}
pub unsafe fn take_1(r: *mut i32) {
    (*r) = 8;
}
pub unsafe fn test_2() {
    let mut v: i32 = 1;
    let cr: *const i32 = &v;
    (unsafe { set_0((cr).cast_mut()) });
    assert!(((v) == (7)));
    (unsafe { take_1((cr).cast_mut()) });
    assert!(((v) == (8)));
    let mut vec_: Vec<i32> = Vec::from([1]);
    let cvec: *const Vec<i32> = &vec_;
    {
        let __a1 = 2;
        (*(&raw const (*cvec)).cast_mut()).push(__a1)
    };
    assert!(((vec_.len()) == (2_usize)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
