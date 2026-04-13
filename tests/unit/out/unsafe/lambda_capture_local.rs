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
    let mut base: i32 = 10;
    let mut factor: i32 = 3;
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            (|x: i32| {
                return ((x) + (base));
            })(_x)
        }) == (15))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 4;
            (|x: i32| {
                return ((x) * (factor));
            })(_x)
        }) == (12))
    );
    base = 100;
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            (|x: i32| {
                return ((x) + (base));
            })(_x)
        }) == (105))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 4;
            (|x: i32| {
                return ((x) * (factor));
            })(_x)
        }) == (12))
    );
    let mut sum: i32 = 0;
    (unsafe {
        let _x: i32 = 1;
        (|x: i32| {
            sum += x;
        })(_x)
    });
    (unsafe {
        let _x: i32 = 2;
        (|x: i32| {
            sum += x;
        })(_x)
    });
    (unsafe {
        let _x: i32 = 3;
        (|x: i32| {
            sum += x;
        })(_x)
    });
    assert!(((sum) == (6)));
    return 0;
}
