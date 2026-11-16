extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn next_id_0() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)).prefix_inc();
}
pub unsafe fn next_id_2() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut last_3)).prefix_inc();
}
static mut last_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut last_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { next_id_0() }) == (1)));
    assert!(((unsafe { next_id_0() }) == (2)));
    assert!(((unsafe { next_id_2() }) == (1)));
    assert!(((unsafe { next_id_0() }) == (3)));
    assert!(((unsafe { next_id_2() }) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
