extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub union anon_0 {
    pub bytes: [u8; 1],
    pub aligner: *mut ::libc::c_void,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct node {
    pub next: *mut node,
    pub x: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: node = node {
        next: std::ptr::null_mut(),
        x: <anon_0>::default(),
    };
    n.next = std::ptr::null_mut();
    (*n.x.bytes.as_mut_ptr().add((0) as usize)) = 171_u8;
    assert!((((((*n.x.bytes.as_mut_ptr().add((0) as usize)) as i32) == (171)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
