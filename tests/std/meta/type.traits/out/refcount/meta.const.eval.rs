extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn pick_0() -> i32 {
    return if false { 1 } else { 2 };
}
pub fn test_1() {
    let mut compile_time: bool = true;
    assert!(compile_time);
    let mut run_time: bool = false;
    assert!(!(run_time));
    assert!((({ pick_0() }) == 2));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
