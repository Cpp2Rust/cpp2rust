extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut version_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn version_2() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut version_calls_0)).prefix_inc();
    return 2;
}
pub static mut revision_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 40 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { version_2() }) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut version_calls_0)) == (1)));
    assert!(((unsafe { version_2() }) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut version_calls_0)) == (2)));
    assert!(
        ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut revision_1)) as *mut i32)
            == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut revision_1)) as *mut i32))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut revision_1)) = 41;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut revision_1)) == (41)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const version_calls_0);
    std::cell::LazyCell::force(&*&raw const revision_1);
}
