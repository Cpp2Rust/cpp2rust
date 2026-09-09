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
pub struct Base {
    pub v: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Derived {
    pub base_Base: Base,
    pub w: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut b: Base = <Base>::default();
    let mut d: Derived = <Derived>::default();
    let mut p: *mut Base = (&mut (*(&mut d as *mut Derived)).base_Base as *mut Base);
    assert!(((unsafe { b.get() }) == (1)));
    assert!(((unsafe { (*p).get() }) == (3)));
    assert!((((*p).v) == (1)));
    return 0;
}
