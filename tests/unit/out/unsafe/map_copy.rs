extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub m: UnsafeMap<i32, (i32, i32)>,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = <S>::default();
    (*&mut (*a.m.get_or_insert_with(1.clone(), || (1, <i32>::default()))).1) = 10;
    let mut b: S = a.clone();
    (*&mut (*b.m.get_or_insert_with(1.clone(), || (1, <i32>::default()))).1) = 20;
    assert!(((*&mut (*a.m.get_or_insert_with(1.clone(), || (1, <i32>::default()))).1) == (10)));
    let mut x: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*&mut (*x.get_or_insert_with(2.clone(), || (2, <i32>::default()))).1) = 3;
    let mut y: UnsafeMap<i32, (i32, i32)> = x.clone();
    (*&mut (*y.get_or_insert_with(2.clone(), || (2, <i32>::default()))).1) = 4;
    assert!(((*&mut (*x.get_or_insert_with(2.clone(), || (2, <i32>::default()))).1) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
