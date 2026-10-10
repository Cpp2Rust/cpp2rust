extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn function_0(mut y: i32, mut z: i32) -> i32 {
    let mut x: i32 = 5;
    return ((x + y) + z);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut y: i32 = ({ function_0(10, 1) });
    assert!((y == 16));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
