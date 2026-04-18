extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
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
            let _y: i32 = 20;
            (|y: i32| {
                return (unsafe {
                    let _z: i32 = 1;
                    (|z: i32| {
                        return (((x) + (y)) + (z));
                    })(_z)
                });
            })(_y)
        }) == (31))
    );
    x = 100;
    assert!(
        ((unsafe {
            let _y: i32 = 20;
            (|y: i32| {
                return (unsafe {
                    let _z: i32 = 1;
                    (|z: i32| {
                        return (((x) + (y)) + (z));
                    })(_z)
                });
            })(_y)
        }) == (121))
    );
    return 0;
}
