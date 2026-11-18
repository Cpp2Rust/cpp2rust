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
pub unsafe fn helper_2() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut global_calls_0)).prefix_inc();
    return 100;
}
pub unsafe fn run_3() -> i32 {
    return (unsafe { helper_4() });
}
pub unsafe fn helper_4() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_calls_1)).prefix_inc();
    return 9;
}
pub unsafe fn test_5() {
    assert!(((unsafe { run_3() }) == (9)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_calls_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_calls_0)) == (0)));
}
pub static mut global_calls_6: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut inner_calls_7: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn base_8() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut global_calls_6)).prefix_inc();
    return 100;
}
pub unsafe fn base_9() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut inner_calls_7)).prefix_inc();
    return 7;
}
pub unsafe fn twice_11() -> i32 {
    return ((unsafe { base_9() }) * (2));
}
pub static mut stored_10: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { ((unsafe { base_9() }) + (1)) });
pub unsafe fn test_12() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut stored_10)) == (8)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_calls_7)) == (1)));
    assert!(((unsafe { twice_11() }) == (14)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut inner_calls_7)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut global_calls_6)) == (0)));
}
pub static mut count_13: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 4 });
pub static mut count_14: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub unsafe fn next_15() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_inc());
}
pub unsafe fn test_16() {
    assert!(((unsafe { next_15() }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_14)) == (100)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_14)) == (100)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_5() });
    (unsafe { test_12() });
    (unsafe { test_16() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_calls_0);
    std::cell::LazyCell::force(&*&raw const inner_calls_1);
    std::cell::LazyCell::force(&*&raw const global_calls_6);
    std::cell::LazyCell::force(&*&raw const inner_calls_7);
    std::cell::LazyCell::force(&*&raw const stored_10);
    std::cell::LazyCell::force(&*&raw const count_13);
    std::cell::LazyCell::force(&*&raw const count_14);
}
