extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub type Step = i32;
pub const Step_first: Step = 2;
pub const Step_second: Step = 10;
pub type Stride = i32;
pub const Stride_single: Stride = 101;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((Step_first as i32) == (2)));
    assert!(((Step_second as i32) == (10)));
    assert!(((Stride_single as i32) == (101)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_0)) == (100)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const first_0);
}
