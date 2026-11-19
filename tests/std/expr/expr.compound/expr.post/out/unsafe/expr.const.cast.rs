extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_reference_const_cast_Pair {
    pub x: i32,
}
pub unsafe fn take_0(r: *mut i32) {
    (*r) = 8;
}
pub unsafe fn test_1() {
    let mut v: i32 = 1;
    let cr: *const i32 = &v;
    (*(&raw const (*cr)).cast_mut()) = 2;
    assert!(((v) == (2)));
    (unsafe { take_0((cr).cast_mut()) });
    assert!(((v) == (8)));
    let mut pair: p4_reference_const_cast_Pair = p4_reference_const_cast_Pair { x: 1 };
    let cpair: *const p4_reference_const_cast_Pair = &pair;
    (*(&raw const (*cpair)).cast_mut()).x = 2;
    assert!(((pair.x) == (2)));
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
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
