extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: BTreeMap<i32, Box<(i32, f64)>> = BTreeMap::new();
    let mut i: i32 = 0;
    let mut k: i32 = 100;
    'loop_: while ((i) < (100)) {
        (*&mut m
            .entry(i.clone())
            .or_insert_with(|| Box::new((i, f64::default())))
            .1) = ((k as f64) / (2_f64));
        {
            i.prefix_inc();
            k.prefix_dec()
        };
    }
    let mut sum: f64 = 0_f64;
    'loop_: for __begin1 in UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<(i32, f64)>>) {
        let i: *const (i32, f64) = &(*__begin1.deref());
        sum += (*i).1;
    }
    'loop_: for __begin1 in UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<(i32, f64)>>) {
        let mut i: (i32, f64) = (*__begin1.deref()).clone();
        sum += (i.0 as f64);
    }
    assert!(((sum) == (7475_f64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
