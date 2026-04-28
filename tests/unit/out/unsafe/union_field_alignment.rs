extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone)]
pub struct node_anon_0 {
    pub bytes: [u8; 1],
    pub aligner: *mut ::libc::c_void,
}
#[derive(Copy, Clone, Default)]
pub struct node {
    pub next: *mut node,
    pub x: node_anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: node = <node>::default();
    n.next = Default::default();
    n.x.bytes[(0) as usize] = 171_u8;
    assert!(((n.x.bytes[(0) as usize] as i32) == (171)));
    return 0;
}
