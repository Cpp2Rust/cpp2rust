extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut shared_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 3 });
pub static mut calls_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasStaticMembers {}
impl HasStaticMembers {
    pub unsafe fn run() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_1)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object_pointer: *mut i32 =
        (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_0)) as *mut i32);
    assert!(((*object_pointer) == (3)));
    (*object_pointer) = 4;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_0)) == (4)));
    let mut function_pointer: Option<unsafe fn()> = (Some(HasStaticMembers::run));
    (unsafe { (function_pointer).unwrap()() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_1)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const shared_0);
    std::cell::LazyCell::force(&*&raw const calls_1);
}
