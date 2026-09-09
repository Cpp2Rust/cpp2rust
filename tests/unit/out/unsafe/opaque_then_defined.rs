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
pub struct list {
    pub head: *mut node,
    pub size: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct node {
    pub value: i32,
    pub next: *mut node,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: node = node {
        value: 42,
        next: std::ptr::null_mut(),
    };
    let mut l: list = list {
        head: (&mut n as *mut node),
        size: 1,
    };
    assert!((((((*l.head).value) == (42)) as i32) != 0));
    assert!(((((l.size) == (1)) as i32) != 0));
    return 0;
}
