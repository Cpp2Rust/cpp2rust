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
    let N: i32 = 5;
    let mut arr1: [i32; 5] = [0, 0, 0, 0, 0];
    let arr2: [i32; 5] = [1, 1, 1, 1, 1];
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        arr1[(i) as usize] = ((i) + (arr2[(i) as usize]));
        i.prefix_inc();
    }
    let mut fatorial: i32 = 1;
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        fatorial *= arr1[(i) as usize];
        i.prefix_inc();
    }
    return fatorial;
}
