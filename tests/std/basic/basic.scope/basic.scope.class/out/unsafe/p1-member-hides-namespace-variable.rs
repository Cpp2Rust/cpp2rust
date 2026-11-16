extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut balance_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 50 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasBalance {
    pub balance: i32,
}
impl HasBalance {
    pub unsafe fn deposit(&mut self, mut amount: i32) -> i32 {
        self.balance += amount;
        return self.balance;
    }
    pub unsafe fn deposit_globally(&mut self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_0)) += amount;
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_0));
    }
}
impl Default for HasBalance {
    fn default() -> Self {
        unsafe { HasBalance { balance: 7 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasBalance = <HasBalance>::default();
    assert!(((unsafe { HasBalance::deposit(&mut object, 3,) }) == (10)));
    assert!(((object.balance) == (10)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut balance_0)) == (50)));
    assert!(((unsafe { HasBalance::deposit_globally(&mut object, 4,) }) == (54)));
    assert!(((object.balance) == (10)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut balance_0)) == (54)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const balance_0);
}
