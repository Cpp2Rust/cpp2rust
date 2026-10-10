extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut p: Ptr<i32>) -> Ptr<i32> {
    return (p.offset((5) as isize));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p1: Ptr<i32> = Ptr::alloc_array((0..10_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    let mut i: u32 = 0_u32;
    'loop_: while (i < 10_u32) {
        elem!(p1, i).write({ (i as i32) });
        i.prefix_inc();
    }
    let mut out: i32 = (elem!(({ foo_0((p1.offset((1) as isize)),) }), 3).read());
    p1.delete();
    assert!((out == 9));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
