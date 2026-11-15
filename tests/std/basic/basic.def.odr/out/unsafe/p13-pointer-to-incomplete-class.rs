extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut x1_0: std::cell::LazyCell<*mut X> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut x2_1: std::cell::LazyCell<*mut X> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub unsafe fn identity_2(mut p: *mut X) -> *mut X {
    return p;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut x1_0)).is_null());
    assert!(
        ((unsafe { identity_2((*std::cell::LazyCell::force_mut(&mut *&raw mut x2_1)),) })
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut x1_0)))
    );
    let mut pp: *mut *mut X =
        (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut x1_0)) as *mut *mut X);
    assert!((*pp).is_null());
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct X;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const x1_0);
    std::cell::LazyCell::force(&*&raw const x2_1);
}
