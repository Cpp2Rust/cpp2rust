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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<i32> = Ptr::alloc_array(Box::new([1, 2]));
    let mut i: usize = p.to_int();
    p.delete();
    return (elem!((<Ptr<i32>>::from_int(i)), 1).read());
}
pub fn __cpp2rust_init_globals() {}
