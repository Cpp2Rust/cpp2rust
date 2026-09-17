extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_0)) + (1))
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_0)) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_1))
            == ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_0)) + (1)))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const first_0);
    std::cell::LazyCell::force(&*&raw const second_1);
}
