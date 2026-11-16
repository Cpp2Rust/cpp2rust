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
pub struct Limit {
    pub value: i32,
}
pub static mut Limit_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 12 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut limit: Limit = Limit { value: 3 };
    assert!(((limit.value) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut Limit_0)) == (12)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut Limit_0)) += limit.value;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut Limit_0)) == (15)));
    assert!(((limit.value) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Limit_0);
}
