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
    let mut N: i32 = 10000;
    let mut arr: Ptr<u32> =
        Ptr::alloc_array((0..(N as usize)).map(|_| 0_u32).collect::<Box<[u32]>>());
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        elem!(arr, i).write(0_u32);
        i.postfix_inc();
    }
    elem!(arr, (N - 1)).write(3148519816_u32);
    let mut words: Ptr<u16> = arr.reinterpret_cast::<u16>();
    assert!((((elem!(words, ((N * 2) - 1)).read()) as i32) == 48042));
    assert!((((elem!(words, ((N * 2) - 2)).read()) as i32) == 39304));
    arr.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
