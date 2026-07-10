extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_0(mut fn_: impl Fn(i32) -> i32, mut x: i32) -> i32 {
    return (unsafe { fn_(x) });
}
pub unsafe fn apply_1(mut fn_: impl Fn(i32) -> i32, mut x: i32) -> i32 {
    return (unsafe { fn_(x) });
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
            apply_0(
                (|x: i32| {
                    return ((x) + (base));
                })
                .clone(),
                5,
            )
        }) == (15))
    );
    base = 100;
    assert!(
        ((unsafe {
            apply_0(
                (|x: i32| {
                    return ((x) + (base));
                })
                .clone(),
                5,
            )
        }) == (105))
    );
    let mut factor: i32 = 3;
    assert!(
        ((unsafe {
            apply_1(
                (|x: i32| {
                    return ((x) * (factor));
                })
                .clone(),
                4,
            )
        }) == (12))
    );
    return 0;
}
