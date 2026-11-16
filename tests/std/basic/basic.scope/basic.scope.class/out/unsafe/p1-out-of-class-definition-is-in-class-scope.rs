extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut initial_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasStaticTotal {
    pub held: i32,
}
impl Default for HasStaticTotal {
    fn default() -> Self {
        unsafe { HasStaticTotal { held: 1 } }
    }
}
pub static mut initial_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 6 });
pub static mut total_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut initial_1)) + (1))
});
impl HasStaticTotal {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        self.held += amount;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_2)) +=
            (*std::cell::LazyCell::force_mut(&mut *&raw mut initial_1));
        return self.held;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_2)) == (7)));
    let mut object: HasStaticTotal = <HasStaticTotal>::default();
    assert!(((unsafe { HasStaticTotal::add(&mut object, 4,) }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_2)) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut initial_1)) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut initial_0)) == (100)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const initial_0);
    std::cell::LazyCell::force(&*&raw const initial_1);
    std::cell::LazyCell::force(&*&raw const total_2);
}
