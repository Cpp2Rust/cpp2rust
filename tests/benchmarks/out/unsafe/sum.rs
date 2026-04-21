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
    let mut N: i64 = 25000000000_i64;
    let mut sum: i64 = 0_i64;
    let mut i: i64 = 0_i64;
    let mut j: i64 = N;
    'loop_: while ((i) < (j)) {
        sum += ((i) + (j));
        i.prefix_inc();
        j.prefix_dec();
    }
    return (sum as i32);
}
