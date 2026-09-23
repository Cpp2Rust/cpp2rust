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
pub struct Probe {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Wrapper_Probe_ {
    pub base_: Probe,
    pub tag: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Wrapper_Probe_ = <Wrapper_Probe_>::default();
    a.tag = 3;
    let mut b: Wrapper_Probe_ = a;
    assert!(((b.tag) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
