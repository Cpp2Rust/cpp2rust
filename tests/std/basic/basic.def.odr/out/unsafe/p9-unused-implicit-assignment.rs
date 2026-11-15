extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut m_ctors_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasUndefinedCopyAssignment {}
impl HasUndefinedCopyAssignment {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut m_ctors_0)).prefix_inc();
        this
    }
}
impl Default for HasUndefinedCopyAssignment {
    fn default() -> Self {
        unsafe { HasUndefinedCopyAssignment::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasMemberWithUndefinedCopyAssignment {
    pub m: HasUndefinedCopyAssignment,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: HasMemberWithUndefinedCopyAssignment =
        <HasMemberWithUndefinedCopyAssignment>::default();
    let mut b: HasMemberWithUndefinedCopyAssignment =
        <HasMemberWithUndefinedCopyAssignment>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut m_ctors_0)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const m_ctors_0);
}
