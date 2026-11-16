extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasStaticCounter {}
impl HasStaticCounter {
    pub unsafe fn bump() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { HasStaticCounter::bump() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (1)));
    (unsafe { HasStaticCounter::bump() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (2)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) = 6;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
