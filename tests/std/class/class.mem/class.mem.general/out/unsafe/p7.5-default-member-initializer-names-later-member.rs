extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut member_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut base_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub unsafe fn scaled_2(mut amount: i32) -> i32 {
    return ((amount) * (1000));
}
pub static mut base_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 4 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasLaterInitializerOperands {
    pub first: i32,
    pub second: i32,
}
impl HasLaterInitializerOperands {
    pub unsafe fn scaled(&self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut member_calls_0)).prefix_inc();
        return ((amount) * (10));
    }
}
impl Default for HasLaterInitializerOperands {
    fn default() -> Self {
        unsafe {
            HasLaterInitializerOperands {
                first: ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_3)) + (1)),
                second: (unsafe {
                    let _amount: i32 = self.first;
                    HasLaterInitializerOperands::scaled(self, _amount)
                }),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasLaterInitializerOperands = <HasLaterInitializerOperands>::default();
    assert!(((object.first) == (5)));
    assert!(((object.second) == (50)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut member_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_1)) == (100)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const member_calls_0);
    std::cell::LazyCell::force(&*&raw const base_1);
    std::cell::LazyCell::force(&*&raw const base_3);
}
