extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut rate_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 3 });
pub unsafe fn scale_1(mut rate: i32) -> i32 {
    rate *= 2;
    return rate;
}
pub unsafe fn read_rate_2() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut rate_0));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { scale_1(5,) }) == (10)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rate_0)) == (3)));
    assert!(((unsafe { read_rate_2() }) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const rate_0);
}
