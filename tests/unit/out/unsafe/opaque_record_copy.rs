extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, Default)]
pub struct Wrapper_Probe_ {
    pub base_: Probe,
    pub tag: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Wrapper_Probe_ = Wrapper_Probe_ {
        base_: Probe {},
        tag: 0_i32,
    };
    a.tag = 3;
    let mut b: Wrapper_Probe_ = a.clone();
    assert!(((b.tag) == (3)));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct Probe;
