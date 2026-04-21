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
    assert!(
        ((unsafe {
            let _arg0: i32 = 3;
            let _arg1: i32 = 4;
            (fn_).unwrap()(_arg0, _arg1)
        }) == (7))
    );
    fn_ = Some(sub_1);
    assert!(
        ((unsafe {
            let _arg0: i32 = 10;
            let _arg1: i32 = 3;
            (fn_).unwrap()(_arg0, _arg1)
        }) == (7))
    );
    fn_ = Some(mul_2);
    assert!(
        ((unsafe {
            let _arg0: i32 = 6;
            let _arg1: i32 = 7;
            (fn_).unwrap()(_arg0, _arg1)
        }) == (42))
    );
    fn_ = None;
    assert!((fn_).is_none());
    fn_ = Some(add_0);
    assert!(!((fn_).is_none()));
    assert!(
        ((unsafe {
            let _arg0: i32 = 1;
            let _arg1: i32 = 1;
            (fn_).unwrap()(_arg0, _arg1)
        }) == (2))
    );
    return 0;
}
