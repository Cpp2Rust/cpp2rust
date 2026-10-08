extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct pair {
    pub a: i32,
    pub b: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 0_i32;
    x = ((x) + (1));
    let mut p: pair = <pair>::default();
    p = pair {
        a: 1,
        b: ((p.a) + (1)),
    };
    return ((x) + (p.b));
}
pub unsafe fn __cpp2rust_init_globals() {}
