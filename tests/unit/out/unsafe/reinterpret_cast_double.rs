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
    let mut d: f64 = 1.0E+0;
    let mut bits: *mut u64 = ((&mut d as *mut f64) as *mut u64);
    assert!(((*bits) == (4607182418800017408_u64)));
    (*bits) = 4614256656552045848_u64;
    assert!(((d) > (3.14E+0)) && ((d) < (3.15E+0)));
    return 0;
}
