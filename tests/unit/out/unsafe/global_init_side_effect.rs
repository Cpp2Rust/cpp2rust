extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {}
impl S {
    pub unsafe fn S(mut x: i32) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) += x;
        this
    }
}
pub static mut a_1: std::cell::LazyCell<S> = std::cell::LazyCell::new(|| unsafe { S::S({ 1 }) });
pub static mut b_2: std::cell::LazyCell<S> = std::cell::LazyCell::new(|| unsafe { S::S({ 10 }) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) == (11)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_0);
    std::cell::LazyCell::force(&*&raw const a_1);
    std::cell::LazyCell::force(&*&raw const b_2);
}
