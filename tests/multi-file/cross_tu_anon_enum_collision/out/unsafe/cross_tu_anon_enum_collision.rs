extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_ALPHA: anon_0 = 7;
pub unsafe fn a_value_1() -> i32 {
    let mut x: i32 = 0;
    x |= (anon_0_ALPHA as i32);
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { a_value_1() }) == (7)) as i32) != 0));
    assert!(((((unsafe { b_value_2() }) == (9)) as i32) != 0));
    return 0;
}
pub type anon_3 = u32;
pub const anon_3_BETA: anon_3 = 9;
pub unsafe fn b_value_2() -> i32 {
    let mut x: i32 = 0;
    x |= (anon_3_BETA as i32);
    return x;
}
