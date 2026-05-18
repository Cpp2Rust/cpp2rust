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
    let mut v: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        {
            let a0_clone = i.clone();
            v.push(a0_clone)
        };
        i.prefix_inc();
    }
    let mut sum: i32 = 0;
    'loop_: for x in 0..(v.len()) {
        let mut x = v[x].clone();
        sum += x;
    }
    return sum;
}
