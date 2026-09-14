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
pub struct S {
    pub a: i32,
}
pub static mut s_0: std::cell::LazyCell<*mut S> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut file_1: std::cell::LazyCell<*mut ::libc::FILE> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut size_2: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe { 0_usize });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)).is_null());
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut file_1)).is_null());
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut size_2)) == (0_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const s_0);
    std::cell::LazyCell::force(&*&raw const file_1);
    std::cell::LazyCell::force(&*&raw const size_2);
}
