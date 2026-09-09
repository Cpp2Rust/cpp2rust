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
    let mut f: f32 = 1.0E+0;
    let mut bits: *mut u32 = ((&mut f as *mut f32) as *mut u32);
    assert!(((*bits) == (1065353216_u32)));
    (*bits) = 1073741824_u32;
    assert!(((f) == (2.0E+0)));
    return 0;
}
