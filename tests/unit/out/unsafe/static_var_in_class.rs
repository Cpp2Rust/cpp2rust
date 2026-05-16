extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
static mut C_inner_const: i32 = 1;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct C {}
impl C {
    pub unsafe fn get(&mut self) -> i32 {
        return C_inner_const;
    }
}
pub static mut S_inner_const: i32 = 2;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: C = <C>::default();
    assert!(((unsafe { c.get() }) == (1)));
    assert!(((S_inner_const) == (2)));
    return 0;
}
