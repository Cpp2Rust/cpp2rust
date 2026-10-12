extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn report_0(mut Err_: i32) -> i32 {
    return (Err_ + 1);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut in_: i32 = 123;
    assert!((in_ == 123));
    let mut Err_: i32 = 1;
    assert!((({ report_0(Err_,) }) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
