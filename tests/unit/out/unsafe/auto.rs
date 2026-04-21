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
    let mut x1: i32 = 1;
    let mut x2: i16 = 2_i16;
    let mut x3: u32 = 4_u32;
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    let mut sum: i32 = 0;
    'loop_: for elem in 0..(v.len()) {
        let mut elem = v[elem].clone();
        sum = ((sum as i32) + elem) as i32;
    }
    return sum;
}
