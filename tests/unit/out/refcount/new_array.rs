extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0(mut p: Ptr<i32>, mut n: i32) -> i32 {
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        total += { (elem!(p, i).read()) };
        i.prefix_inc();
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut array: Ptr<i32> =
        Ptr::alloc_array((0..100_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    array.delete();
    let mut filled: Ptr<i32> =
        Ptr::alloc_array((0..4_usize).map(|_| 0_i32).collect::<Box<[i32]>>());
    let mut i: i32 = 0;
    'loop_: while (i < 4) {
        elem!(filled, i).write({ (i + 1) });
        i.prefix_inc();
    }
    if (({
        let _p: Ptr<i32> = (filled).clone();
        sum_0(_p, 4)
    }) != 10)
    {
        return 1;
    }
    filled.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
