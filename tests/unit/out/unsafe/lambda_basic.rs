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
    assert!(
        ((unsafe {
            (|| {
                return 42;
            })()
        }) == (42))
    );
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((x) + (1));
            })(1)
        }) == (2))
    );
    assert!(
        ((unsafe {
            (|x: i32, y: i32, z: i32| {
                return ((((x) * (100)) + ((y) * (10))) + (z));
            })(1, 2, 3)
        }) == (123))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
