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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: BTreeMap<i32, Box<f64>> = BTreeMap::new();
    let mut i: i32 = 0;
    let mut k: i32 = 100;
    'loop_: while ((i) < (100)) {
        (*m.entry(i).or_default().as_mut()) = ((k as f64) / (2.0E+0));
        i.prefix_inc();
        k.prefix_dec();
    }
    let mut sum: f64 = 0_f64;
    'loop_: for i in UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<f64>>) {
        sum += *i.second();
    }
    'loop_: for i in UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<f64>>) {
        sum += (*i.first() as f64);
    }
    return (sum as i32);
}
