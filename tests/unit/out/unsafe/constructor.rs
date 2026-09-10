extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Clone, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn const_method(&self) -> i32 {
        return ((self.v) * (2));
    }
    pub unsafe fn mut_method(&mut self) {
        self.v += 1;
    }
    pub unsafe fn S(mut init: i32) -> Self {
        let mut this = Self { v: init };
        (unsafe { S::mut_method(&mut this) });
        total_0 += (unsafe { S::const_method(&mut this) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { S::mut_method(self) });
        total_0 += (unsafe { S::const_method(self) });
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut s: S = S::S({ 3 });
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
        assert!(((s.v) == (4)));
        assert!(((total_0) == (8)));
    }
    assert!(((total_0) == (18)));
    return 0;
}
