extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(
    mut a1: i32,
    mut a2: i32,
    mut a3: i32,
    mut a4: i32,
    mut a5: i32,
    mut a6: i32,
    mut a7: i32,
    mut a8: i32,
    mut a9: i32,
    mut a10: i32,
    mut a11: i32,
    mut a12: i32,
    mut a13: i32,
    mut a14: i32,
) -> i32 {
    return 22;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: Option<
        unsafe fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32,
    > = (Some(foo_0));
    assert!(((unsafe { (f).unwrap()(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,) }) == (22)));
    return 0;
}
