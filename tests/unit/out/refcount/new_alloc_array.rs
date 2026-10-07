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
    let mut array: Ptr<i32> =
        Ptr::alloc_array((0..100_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    {
        (array).to_any().memset(
            (0) as u8,
            (4usize as usize).wrapping_mul(100_usize) as usize,
        );
        (array).to_any()
    };
    elem!(array, 99).write(-1_i32);
    let mut p1: Ptr<i32> = (array).clone();
    'loop_: while ((p1.read()) >= 0) {
        p1.write(1);
        p1.prefix_inc();
    }
    let mut out: i32 = 0;
    let mut p1: Ptr<i32> = (array).clone();
    'loop_: while ((p1.read()) >= 0) {
        out += { (p1.read()) };
        p1.prefix_inc();
    }
    let mut p2: Ptr<i32> = (array).clone();
    p2.delete();
    assert!((out == 99));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
