extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut inner_then_runs_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut inner_else_runs_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut outer_else_runs_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    if (choice != 0) {
        if (choice != 0) {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_then_runs_0)).prefix_inc();
        } else {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_else_runs_1)).prefix_inc();
            assert!(((choice) == (0)));
        }
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outer_else_runs_2)).prefix_inc();
        assert!(((choice) == (0)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_then_runs_0)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_else_runs_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut outer_else_runs_2)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const inner_then_runs_0);
    std::cell::LazyCell::force(&*&raw const inner_else_runs_1);
    std::cell::LazyCell::force(&*&raw const outer_else_runs_2);
}
