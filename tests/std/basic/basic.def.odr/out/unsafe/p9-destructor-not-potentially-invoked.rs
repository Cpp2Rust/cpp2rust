extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut t_ctors_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasUndefinedDestructor {}
impl HasUndefinedDestructor {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut t_ctors_0)).prefix_inc();
        this
    }
}
impl Default for HasUndefinedDestructor {
    fn default() -> Self {
        unsafe { HasUndefinedDestructor::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut HasUndefinedDestructor =
        (Box::leak(Box::new(HasUndefinedDestructor::new())) as *mut HasUndefinedDestructor);
    assert!(!((p).is_null()));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut t_ctors_0)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const t_ctors_0);
}
