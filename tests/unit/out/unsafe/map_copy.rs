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
    (*a.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))) = 10;
    let mut b: UnsafeMap<i32, (i32, i32)> = a.clone();
    (*b.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))) = 20;
    assert!(((*a.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))) == (10)));
    let mut x: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*x.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))) = 3;
    let mut y: UnsafeMap<i32, (i32, i32)> = x.clone();
    (*y.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))) = 4;
    assert!(((*x.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
