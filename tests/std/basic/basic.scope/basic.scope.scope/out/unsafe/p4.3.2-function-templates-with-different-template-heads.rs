extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut type_parameter_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut value_parameter_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn describe_2(mut value: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut type_parameter_calls_0)).prefix_inc();
    return ((value) + (10));
}
pub unsafe fn describe_3() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut value_parameter_calls_1)).prefix_inc();
    return ((3) + (20));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { describe_2(3,) }) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut type_parameter_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut value_parameter_calls_1)) == (0)));
    assert!(((unsafe { describe_3() }) == (23)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut type_parameter_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut value_parameter_calls_1)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const type_parameter_calls_0);
    std::cell::LazyCell::force(&*&raw const value_parameter_calls_1);
}
