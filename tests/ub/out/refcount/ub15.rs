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
    let mut arr: Ptr<i32> = Ptr::alloc_array((0..15_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    let mut ptr: Ptr<i32> = arr.offset((15) as isize);
    let mut out: i32 = (ptr.read());
    arr.delete();
    return out;
}
pub fn __cpp2rust_init_globals() {}
