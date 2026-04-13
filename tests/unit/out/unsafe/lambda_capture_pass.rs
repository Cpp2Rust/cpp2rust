extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
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
            let _x: i32 = 5;
            apply_0(_fn, _x)
        }) == (15))
    );
    base = 100;
    assert!(
        ((unsafe {
            let _fn: _ = (|x: i32| {
                return ((x) + (base));
            })
            .clone();
            let _x: i32 = 5;
            apply_0(_fn, _x)
        }) == (105))
    );
    let mut factor: i32 = 3;
    assert!(
        ((unsafe {
            let _fn: _ = (|x: i32| {
                return ((x) * (factor));
            })
            .clone();
            let _x: i32 = 4;
            apply_1(_fn, _x)
        }) == (12))
    );
    return 0;
}
