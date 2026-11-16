extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut function_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Reading {
    pub value: i32,
}
pub unsafe fn Reading_1(mut raw_: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_0)).prefix_inc();
    return ((raw_) + (1));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut reading: Reading = Reading { value: 4 };
    assert!(((reading.value) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_0)) == (0)));
    assert!(((unsafe { Reading_1(reading.value,) }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_0)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const function_calls_0);
}
