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
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    let mut sum: i32 = 0;
    'loop_: for x in 0..(v1.len()) {
        let mut x = v1[x].clone();
        sum += x.prefix_inc();
    }
    'loop_: for x in 0..(v1.len()) {
        let x = v1[x].clone();
        sum += x;
    }
    'loop_: for x in 0..(v1.len()) {
        let mut x = v1.as_mut_ptr().add(x);
        (*x) = (((*x) as i32) + 10) as i32;
    }
    'loop_: for x in 0..(v1.len()) {
        let mut x = v1.as_ptr().add(x);
        sum += (*x);
    }
    let mut v2: Vec<*mut i32> = Vec::new();
    v2.push((&mut v1[(0_u64) as usize] as *mut i32));
    v2.push((&mut v1[(1_u64) as usize] as *mut i32));
    v2.push((&mut v1[(2_u64) as usize] as *mut i32));
    'loop_: for p in 0..(v2.len()) {
        let mut p = v2[p].clone();
        (*p) += 5;
    }
    'loop_: for p in 0..(v2.len()) {
        let p = v2[p].clone();
        sum += (*p);
    }
    'loop_: for p in 0..(v2.len()) {
        let mut p = v2[p].clone();
        (*p) = (((*p) as i32) + 5) as i32;
    }
    'loop_: for p in 0..(v2.len()) {
        let mut p = v2[p].clone();
        sum += (*p);
    }
    return sum;
}
