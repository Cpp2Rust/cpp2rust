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
    let mut d: f64 = 1_f64;
    let mut bits: *mut u64 = ((&mut d as *mut f64) as *mut u64);
    assert!(((*bits) == (4607182418800017408_u64)));
    (*bits) = 4614256656552045848_u64;
    assert!(((d) > (3.14_f64)) && ((d) < (3.15_f64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
