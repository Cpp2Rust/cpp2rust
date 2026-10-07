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
    let mut N: i32 = 3;
    let mut arr: Ptr<i32> =
        Ptr::alloc_array((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>());
    {
        (arr).to_any().memset(
            (1) as u8,
            (4usize as usize).wrapping_mul((N as usize)) as usize,
        );
        (arr).to_any()
    };
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        sum += { (elem!(arr, i).read()) };
        i.prefix_inc();
    }
    arr.delete();
    assert!((sum == 50529027));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
