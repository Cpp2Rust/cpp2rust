extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut element: Ptr<i32> = Ptr::alloc(10);
    let mut ptr: Ptr<i32> = element.offset((1) as isize);
    let mut out: i32 = (ptr.read());
    element.delete();
    return out;
}
pub fn __cpp2rust_init_globals() {}
