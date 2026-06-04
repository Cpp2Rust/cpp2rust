extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_0(mut fn_: impl Fn(i32) -> i32, mut x: i32) -> i32 {
    return (unsafe {
        let _x: i32 = x;
        fn_(_x)
    });
}
pub unsafe fn apply_1(mut fn_: impl Fn(i32) -> i32, mut x: i32) -> i32 {
    return (unsafe {
        let _x: i32 = x;
        fn_(_x)
    });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut base: i32 = 10;
    assert!(
        ((unsafe {
            let _fn: _ = (|x: i32| {
                return ((x) + (base));
            })
            .clone();
            apply_0(_fn, 5)
        }) == (15))
    );
    base = 100;
    assert!(
        ((unsafe {
            let _fn: _ = (|x: i32| {
                return ((x) + (base));
            })
            .clone();
            apply_0(_fn, 5)
        }) == (105))
    );
    let mut factor: i32 = 3;
    assert!(
        ((unsafe {
            let _fn: _ = (|x: i32| {
                return ((x) * (factor));
            })
            .clone();
            apply_1(_fn, 4)
        }) == (12))
    );
    return 0;
}
