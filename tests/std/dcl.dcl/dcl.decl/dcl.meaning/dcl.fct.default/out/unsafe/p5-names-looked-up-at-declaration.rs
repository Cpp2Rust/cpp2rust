extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut scaled_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut rate_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
pub unsafe fn scaled_2(mut amount: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut scaled_calls_0)).prefix_inc();
    return ((amount) * (10));
}
pub unsafe fn total_3(mut base: Option<i32>) -> i32 {
    let mut base: i32 = base.unwrap_or_else(|| unsafe {
        (unsafe { scaled_2((*std::cell::LazyCell::force_mut(&mut *&raw mut rate_1))) })
    });
    return base;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut rate_1)) = 2;
    {
        let mut rate: i32 = 3;
        assert!(((unsafe { total_3(None,) }) == (20)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut scaled_calls_0)) == (1)));
        assert!(((rate) == (3)));
    }
    assert!(((unsafe { total_3(Some(5),) }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut scaled_calls_0)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const scaled_calls_0);
    std::cell::LazyCell::force(&*&raw const rate_1);
}
