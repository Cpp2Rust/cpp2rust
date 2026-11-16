extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut stock_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub unsafe fn minimum_1() -> i32 {
    return 50;
}
pub type anon_2 = u32;
pub const anon_2_reserve: anon_2 = 30;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasLaterMembers {
    pub stock: i32,
}
impl HasLaterMembers {
    pub unsafe fn take(&mut self, mut amount: i32) -> i32 {
        self.stock -= amount;
        return (((self.stock) + (unsafe { HasLaterMembers::minimum(self) }))
            + (anon_2_reserve as i32));
    }
    pub unsafe fn minimum(&self) -> i32 {
        return 2;
    }
}
impl Default for HasLaterMembers {
    fn default() -> Self {
        unsafe { HasLaterMembers { stock: 9 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasLaterMembers = <HasLaterMembers>::default();
    assert!(((unsafe { HasLaterMembers::take(&mut object, 3,) }) == (38)));
    assert!(((object.stock) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut stock_0)) == (100)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const stock_0);
}
