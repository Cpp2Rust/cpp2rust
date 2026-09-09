extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_small_0() -> bool {
    return true;
}
pub unsafe fn is_small_1() -> bool {
    return false;
}
pub unsafe fn pick_2(mut x: i32) -> i32 {
    if (true) && (true) {
        return 1;
    }
    return 2;
}
pub unsafe fn pick_3(mut x: i64) -> i32 {
    if (true) && (false) {
        return 1;
    }
    return 2;
}
pub unsafe fn pick_4(mut x: f32) -> i32 {
    if (false) && (true) {
        return 1;
    }
    return 2;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((unsafe { is_small_0() }));
    assert!(!(unsafe { is_small_1() }));
    assert!(((unsafe { pick_2(1,) }) == (1)));
    assert!(((unsafe { pick_3(1_i64,) }) == (2)));
    assert!(((unsafe { pick_4(1.0E+0,) }) == (2)));
    return 0;
}
