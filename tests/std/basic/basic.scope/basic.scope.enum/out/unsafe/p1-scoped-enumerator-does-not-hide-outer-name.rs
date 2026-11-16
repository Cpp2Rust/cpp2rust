extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Power = i32;
pub const Power_off: Power = 3;
pub const Power_on: Power = 8;
pub static mut off_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 40 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut off_0)) == (40)));
    assert!(((Power_off as i32) == (3)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut off_0)) += (Power_on as i32);
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut off_0)) == (48)));
    assert!(((Power_off as i32) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const off_0);
}
