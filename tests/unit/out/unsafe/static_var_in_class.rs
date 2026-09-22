extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
static mut inner_const_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct C {}
impl C {
    pub unsafe fn get(&mut self) -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_const_0));
    }
}
pub static mut inner_const_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 2 });
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: C = C {};
    assert!(((unsafe { C::get(&mut c,) }) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_const_1)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const inner_const_0);
    std::cell::LazyCell::force(&*&raw const inner_const_1);
}
