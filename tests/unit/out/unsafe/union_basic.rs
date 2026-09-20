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
pub union basic {
    pub i: i32,
    pub f: f32,
}
impl Default for basic {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union empty {
    __empty: u8,
}
impl Default for empty {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut u: basic = <basic>::default();
    let mut e: empty = <empty>::default();
    &(e);
    u.i = 42;
    assert!(((u.i) == (42)));
    u.f = 3.140000105E+0;
    assert!(((u.f) == (3.140000105E+0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
