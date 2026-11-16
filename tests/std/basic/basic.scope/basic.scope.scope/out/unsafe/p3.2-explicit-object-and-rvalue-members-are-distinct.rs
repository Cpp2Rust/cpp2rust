extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut explicit_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut rvalue_calls_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasExplicitObjectOverload {
    pub base: i32,
}
impl HasExplicitObjectOverload {
    pub unsafe fn pick_1(&mut self, self_: *mut HasExplicitObjectOverload, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut explicit_calls_0)).prefix_inc();
        return (((*self_).base) + (amount));
    }
    pub unsafe fn pick_2(&mut self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)).prefix_inc();
        return (((self.base) + (amount)) + (100));
    }
}
impl Default for HasExplicitObjectOverload {
    fn default() -> Self {
        unsafe { HasExplicitObjectOverload { base: 10 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasExplicitObjectOverload = <HasExplicitObjectOverload>::default();
    assert!(((unsafe { pick(&mut object, 1,) }) == (11)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut explicit_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)) == (0)));
    assert!(
        ((unsafe {
            HasExplicitObjectOverload::pick_2(&mut <HasExplicitObjectOverload>::default(), 2)
        }) == (112))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut explicit_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const explicit_calls_0);
    std::cell::LazyCell::force(&*&raw const rvalue_calls_1);
}
