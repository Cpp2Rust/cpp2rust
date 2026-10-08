extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut c_0: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe { 5_usize });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
pub static mut table_size_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 256_usize });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Table_char_ {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pa: *const usize =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut c_0)) as *const usize);
    assert!((((*pa).wrapping_add(1_usize)) == (6_usize)));
    let mut G: *const usize =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut table_size_1)) as *const usize);
    assert!(((*G) >= (256_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const c_0);
    std::cell::LazyCell::force(&*&raw const table_size_1);
}
