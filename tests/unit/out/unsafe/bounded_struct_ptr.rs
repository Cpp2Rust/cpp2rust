extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Foo {
    pub x1: i32,
    pub x2: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [Foo; 2] = [Foo { x1: 1, x2: 2 }, Foo { x1: 3, x2: 4 }];
    let mut p1: *mut i32 = (&mut arr[(1) as usize].x1 as *mut i32);
    let mut a: i32 = (*p1);
    let mut p2: *mut Foo = (&mut arr[(0) as usize] as *mut Foo);
    return ((a) + ((*p2).x2));
}
