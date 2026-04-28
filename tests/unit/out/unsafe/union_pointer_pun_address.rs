extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct node_a {
    pub n: i32,
}
#[derive(Copy, Clone, Default)]
pub struct node_b {
    pub data: *mut ::libc::c_void,
    pub next: *mut node_b,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: node_a = node_a { n: 123 };
    #[derive(Copy, Clone, Default)]
    pub struct anon_0 {
        pub to_a: *mut node_a,
        pub to_b: *mut node_b,
    };
    let mut ptr: anon_0 = <anon_0>::default();
    ptr.to_a = (&mut a as *mut node_a);
    let mut out: *mut node_b = ptr.to_b;
    assert!(((out as *mut ::libc::c_void) == ((&mut a as *mut node_a) as *mut ::libc::c_void)));
    return 0;
}
