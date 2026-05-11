extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

// a.rs
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((unsafe {
            let _x: i32 = 42;
            helper_0(_x)
        }) == (43)) as i32)
            != 0)
    );
    return 0;
}
// b.rs
pub unsafe fn unrelated1_1() -> i32 {
    return 1;
}
pub unsafe fn unrelated2_2() -> i32 {
    return 2;
}
pub unsafe fn unrelated3_3() -> i32 {
    return 3;
}
pub unsafe fn helper_4(mut x: i32) -> i32 {
    (unsafe { unrelated1_1() });
    (unsafe { unrelated2_2() });
    (unsafe { unrelated3_3() });
    return ((x) + (1));
}
