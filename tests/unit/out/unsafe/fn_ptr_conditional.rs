extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inc_0(mut x: i32) -> i32 {
    return ((x) + (1));
}
pub unsafe fn dec_1(mut x: i32) -> i32 {
    return ((x) - (1));
}
pub unsafe fn identity_2(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn pick_3(mut mode: i32) -> Option<unsafe fn(i32) -> i32> {
    return if ((mode) > (0)) {
        Some(inc_0)
    } else {
        if ((mode) < (0)) {
            Some(dec_1)
        } else {
            Some(identity_2)
        }
    };
}
pub unsafe fn apply_4(mut fn_: Option<unsafe fn(i32) -> i32>, mut x: i32) -> i32 {
    let mut actual: Option<unsafe fn(i32) -> i32> = if !(fn_).is_none() {
        fn_
    } else {
        Some(identity_2)
    };
    return (unsafe { (actual).unwrap()(x) });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { (unsafe { pick_3(1,) }).unwrap()(10,) }) == (11)));
    assert!(((unsafe { (unsafe { pick_3(-1_i32,) }).unwrap()(10,) }) == (9)));
    assert!(((unsafe { (unsafe { pick_3(0,) }).unwrap()(10,) }) == (10)));
    assert!(((unsafe { apply_4(Some(inc_0), 5,) }) == (6)));
    assert!(((unsafe { apply_4(None, 5,) }) == (5)));
    return 0;
}
