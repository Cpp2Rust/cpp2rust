extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut same_name_different_type_0: i32 = unsafe { 1 };
pub static mut same_name_same_type_1: i32 = unsafe { 5 };
pub unsafe fn a_foo_2() -> i32 {
    return (*&raw mut same_name_different_type_0);
}
pub unsafe fn a_bar_3() -> i32 {
    return (*&raw mut same_name_same_type_1);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { a_foo_2() }) == (1)) as i32) != 0));
    assert!(((((unsafe { b_foo_4() }) == (1.0E+0)) as i32) != 0));
    assert!(((((unsafe { a_bar_3() }) == (5)) as i32) != 0));
    assert!(((((unsafe { b_bar_5() }) == (6)) as i32) != 0));
    return 0;
}
pub static mut same_name_different_type_6: f32 = unsafe { 1.0E+0 };
pub static mut same_name_same_type_7: i32 = unsafe { 6 };
pub unsafe fn b_foo_4() -> f32 {
    return (*&raw mut same_name_different_type_6);
}
pub unsafe fn b_bar_5() -> i32 {
    return (*&raw mut same_name_same_type_7);
}
