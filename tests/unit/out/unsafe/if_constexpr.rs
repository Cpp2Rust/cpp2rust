extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn classify_0(mut x: *mut i32) -> i32 {
    {
        return (*x);
    }
    return 1;
}
pub unsafe fn classify_1(mut x: i64) -> i32 {
    {
        return 2;
    }
    return 1;
}
pub unsafe fn classify_2(mut x: i32) -> i32 {
    return 1;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: i32 = 7;
    assert!(((unsafe { classify_0((&mut v as *mut i32),) }) == (7)));
    assert!(((unsafe { classify_1(1_i64,) }) == (2)));
    assert!(((unsafe { classify_2(1,) }) == (1)));
    return 0;
}
