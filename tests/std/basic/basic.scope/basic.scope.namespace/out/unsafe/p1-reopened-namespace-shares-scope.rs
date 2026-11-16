extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 4 });
pub static mut count_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub unsafe fn next_2() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { next_2() }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_1)) == (100)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_1)) == (100)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count_1);
}
