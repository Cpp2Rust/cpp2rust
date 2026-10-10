extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut a: Ptr<i32>) -> Ptr<i32> {
    return (a.offset((5) as isize));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p1: Ptr<i32> = Ptr::alloc_array((0..10_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    let mut out: i32 = (elem!(({ foo_0((p1.offset((1) as isize)),) }), 4).read());
    p1.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
