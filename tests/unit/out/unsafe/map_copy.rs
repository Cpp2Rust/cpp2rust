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
    let mut a: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*a.get_or_insert_with(1.clone(), || (1, <i32>::default()))).1 = 10;
    let mut b: UnsafeMap<i32, (i32, i32)> = a.clone();
    (*b.get_or_insert_with(1.clone(), || (1, <i32>::default()))).1 = 20;
    assert!((((*a.get_or_insert_with(1.clone(), || (1, <i32>::default()))).1) == (10)));
    let mut x: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*x.get_or_insert_with(2.clone(), || (2, <i32>::default()))).1 = 3;
    let mut y: UnsafeMap<i32, (i32, i32)> = x.clone();
    (*y.get_or_insert_with(2.clone(), || (2, <i32>::default()))).1 = 4;
    assert!((((*x.get_or_insert_with(2.clone(), || (2, <i32>::default()))).1) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
