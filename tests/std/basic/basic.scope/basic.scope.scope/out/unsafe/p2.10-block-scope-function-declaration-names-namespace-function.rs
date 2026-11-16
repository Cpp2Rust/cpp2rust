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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut measure: i32 = 5;
    {
        assert!(((unsafe { measure_1() }) == (7)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_0)) == (1)));
    }
    assert!(((measure) == (5)));
    return 0;
}
pub unsafe fn measure_1() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_0)).prefix_inc();
    return 7;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const function_calls_0);
}
