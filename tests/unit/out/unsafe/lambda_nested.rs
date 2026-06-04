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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 10;
    assert!(
        ((unsafe {
            (|y: i32| {
                return (unsafe {
                    (|z: i32| {
                        return (((x) + (y)) + (z));
                    })(1)
                });
            })(20)
        }) == (31))
    );
    x = 100;
    assert!(
        ((unsafe {
            (|y: i32| {
                return (unsafe {
                    (|z: i32| {
                        return (((x) + (y)) + (z));
                    })(1)
                });
            })(20)
        }) == (121))
    );
    return 0;
}
