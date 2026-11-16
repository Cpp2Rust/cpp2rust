extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut null_pm: isize = -1_isize;
    let mut first_pm: isize = (::std::mem::offset_of!(HasTwoInts, first) as isize);
    assert!(((first_pm) != (null_pm)));
    assert!(!((first_pm) == (null_pm)));
    assert!(((null_pm) != (first_pm)));
    assert!(!((null_pm) == (first_pm)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
