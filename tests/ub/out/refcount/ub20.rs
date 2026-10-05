extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut single: Ptr<i32>) {
    single.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: Ptr<i32> = Ptr::alloc_array((0..10_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    ({ foo_0((x).clone()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
