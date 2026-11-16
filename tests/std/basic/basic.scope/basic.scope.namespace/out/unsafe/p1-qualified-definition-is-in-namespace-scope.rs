extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut global_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut inner_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn base_2() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut global_calls_0)).prefix_inc();
    return 100;
}
pub unsafe fn base_3() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_calls_1)).prefix_inc();
    return 7;
}
pub unsafe fn twice_5() -> i32 {
    return ((unsafe { base_3() }) * (2));
}
pub static mut stored_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { ((unsafe { base_3() }) + (1)) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut stored_4)) == (8)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_calls_1)) == (1)));
    assert!(((unsafe { twice_5() }) == (14)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_calls_1)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_calls_0)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_calls_0);
    std::cell::LazyCell::force(&*&raw const inner_calls_1);
    std::cell::LazyCell::force(&*&raw const stored_4);
}
