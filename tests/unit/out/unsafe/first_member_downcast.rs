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
pub struct base {
    pub kind: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct derived {
    pub head: base,
    pub value: usize,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: *mut derived =
        (libcc2rs::malloc_unsafe(::std::mem::size_of::<derived>()) as *mut derived);
    assert!((!((d).is_null())));
    (*d).head.kind = 3;
    (*d).value = 7_usize;
    let mut b: *mut base = (&mut (*d).head as *mut base);
    let mut back: *mut derived = (b as *mut derived);
    assert!(((back) == (d)));
    assert!((((*back).value) == (7_usize)));
    assert!((((*back).head.kind) == (3)));
    (*back).value = 8_usize;
    assert!((((*d).value) == (8_usize)));
    (*b).kind = 4;
    assert!((((*d).head.kind) == (4)));
    libcc2rs::free_unsafe((back as *mut ::libc::c_void));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
