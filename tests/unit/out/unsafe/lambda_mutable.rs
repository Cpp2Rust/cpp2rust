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
    let mut start: i32 = 5;
    assert!(
        ((unsafe {
            (|| {
                return start.postfix_inc();
            })()
        }) == (5))
    );
    assert!(
        ((unsafe {
            (|| {
                return start.postfix_inc();
            })()
        }) == (6))
    );
    assert!(
        ((unsafe {
            (|| {
                return start.postfix_inc();
            })()
        }) == (7))
    );
    assert!(((start) == (5)));
    let mut total: i32 = 0;
    assert!(
        ((unsafe {
            (|x: i32| {
                total += x;
                return total;
            })(1)
        }) == (1))
    );
    assert!(
        ((unsafe {
            (|x: i32| {
                total += x;
                return total;
            })(2)
        }) == (3))
    );
    assert!(((total) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
