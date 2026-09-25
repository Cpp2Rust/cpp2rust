extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 3;
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((((a) + (b)) + (c)) + (x));
            })(10)
        }) == (16))
    );
    a = 100;
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((((a) + (b)) + (c)) + (x));
            })(10)
        }) == (16))
    );
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((((a) + (b)) + (c)) + (x));
            })(10)
        }) == (115))
    );
    b = 200;
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((((a) + (b)) + (c)) + (x));
            })(10)
        }) == (313))
    );
    assert!(
        ((unsafe {
            (|x: i32| {
                c += x;
                return (((a) + (b)) + (c));
            })(1)
        }) == (((100) + (200)) + (4)))
    );
    assert!(((c) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
