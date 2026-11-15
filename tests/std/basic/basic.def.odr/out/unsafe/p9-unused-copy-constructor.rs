extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut ctor_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct HasUndefinedCopyCtor {
    pub value: i32,
}
impl HasUndefinedCopyCtor {
    pub unsafe fn new() -> Self {
        let mut this = Self { value: 3 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctor_calls_0)).prefix_inc();
        this
    }
}
impl Default for HasUndefinedCopyCtor {
    fn default() -> Self {
        unsafe { HasUndefinedCopyCtor::new() }
    }
}
pub unsafe fn read_1(object: *const HasUndefinedCopyCtor) -> i32 {
    return (*object).value;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: HasUndefinedCopyCtor = HasUndefinedCopyCtor::new();
    assert!(((unsafe { read_1(&a,) }) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctor_calls_0)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctor_calls_0);
}
