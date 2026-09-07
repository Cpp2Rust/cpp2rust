extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn add_0(mut a: i32, mut b: i32) -> i32 {
    return ((a) + (b));
}
pub unsafe fn sub_1(mut a: i32, mut b: i32) -> i32 {
    return ((a) - (b));
}
pub unsafe fn mul_2(mut a: i32, mut b: i32) -> i32 {
    return ((a) * (b));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn_: Option<unsafe fn(i32, i32) -> i32> = Some(add_0);
    assert!(((unsafe { (fn_).unwrap()(3, 4,) }) == (7)));
    fn_ = Some(sub_1);
    assert!(((unsafe { (fn_).unwrap()(10, 3,) }) == (7)));
    fn_ = Some(mul_2);
    assert!(((unsafe { (fn_).unwrap()(6, 7,) }) == (42)));
    fn_ = None;
    assert!((fn_).is_none());
    fn_ = Some(add_0);
    assert!(!((fn_).is_none()));
    assert!(((unsafe { (fn_).unwrap()(1, 1,) }) == (2)));
    return 0;
}
