extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut step_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub static mut step_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 4 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasStaticStep {
    pub total: i32,
}
impl HasStaticStep {
    pub unsafe fn advance(&mut self, mut amount: Option<i32>) -> i32 {
        let mut amount: i32 = amount
            .unwrap_or_else(|| unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut step_1)) });
        self.total += amount;
        return self.total;
    }
}
impl Default for HasStaticStep {
    fn default() -> Self {
        unsafe { HasStaticStep { total: 1 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasStaticStep = <HasStaticStep>::default();
    assert!(((unsafe { HasStaticStep::advance(&mut object, None,) }) == (5)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut step_1)) = 6;
    assert!(((unsafe { HasStaticStep::advance(&mut object, None,) }) == (11)));
    assert!(((unsafe { HasStaticStep::advance(&mut object, Some(2),) }) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut step_0)) == (100)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const step_0);
    std::cell::LazyCell::force(&*&raw const step_1);
}
