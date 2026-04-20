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
    let mut p1: (f64, i32) = (1.into(), 2.into());
    assert!(((p1.0) == (1.0E+0)));
    assert!(((p1.1) == (2)));
    let mut p2: (f64, i32) = (3.into(), 4.into());
    assert!(((p2.0) == (3.0E+0)));
    assert!(((p2.1) == (4)));
    let mut p3: (f64, i32) = (5.into(), 6.into());
    assert!(((p3.0) == (5.0E+0)));
    assert!(((p3.1) == (6)));
    return 0;
}
