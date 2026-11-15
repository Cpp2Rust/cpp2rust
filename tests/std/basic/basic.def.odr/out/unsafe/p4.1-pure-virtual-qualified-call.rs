extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait HasPureVirtual {
    unsafe fn f(&mut self);
}
impl HasPureVirtual {
    unsafe fn f(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesPureVirtual {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: OverridesPureVirtual = <OverridesPureVirtual>::default();
    let mut p: *mut dyn HasPureVirtual = (&mut d as *mut OverridesPureVirtual);
    (unsafe { (*p).f() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1)));
    (unsafe { (d as HasPureVirtual).f() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (2)));
    return 0;
}
unsafe impl HasPureVirtual for OverridesPureVirtual {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)).prefix_inc();
        (unsafe { self.f() });
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const derived_calls_1);
}
