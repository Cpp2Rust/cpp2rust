extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn call_0(mut f: Option<unsafe fn(i32, i32) -> i32>, mut a: i32, mut b: i32) -> i32 {
    return (unsafe {
        let _arg0: i32 = a;
        let _arg1: i32 = b;
        (f).unwrap()(_arg0, _arg1)
    });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut add: Option<unsafe fn(i32, i32) -> i32> = Some(|a: i32, b: i32| {
        return ((a) + (b));
    });
    let mut sub: Option<unsafe fn(i32, i32) -> i32> = Some(|a: i32, b: i32| {
        return ((a) - (b));
    });
    assert!(!((add).is_none()));
    assert!(((add) != (sub)));
    assert!(
        ((unsafe {
            let _arg0: i32 = 2;
            let _arg1: i32 = 3;
            (add).unwrap()(_arg0, _arg1)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _arg0: i32 = 10;
            let _arg1: i32 = 4;
            (sub).unwrap()(_arg0, _arg1)
        }) == (6))
    );
    assert!(
        ((unsafe {
            let _f: Option<unsafe fn(i32, i32) -> i32> = add;
            let _a: i32 = 7;
            let _b: i32 = 8;
            call_0(_f, _a, _b)
        }) == (15))
    );
    assert!(
        ((unsafe {
            let _f: Option<unsafe fn(i32, i32) -> i32> = Some(|a: i32, b: i32| {
                return ((a) * (b));
            });
            let _a: i32 = 6;
            let _b: i32 = 7;
            call_0(_f, _a, _b)
        }) == (42))
    );
    return 0;
}
