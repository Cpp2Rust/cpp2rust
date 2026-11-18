extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0() -> i32 {
    return if false { 1 } else { 2 };
}
pub unsafe fn test_1() {
    let compile_time: bool = true;
    assert!(compile_time);
    let mut run_time: bool = false;
    assert!(!(run_time));
    assert!(((unsafe { pick_0() }) == (2)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
