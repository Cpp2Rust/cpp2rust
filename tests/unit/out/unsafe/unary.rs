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
    let mut x: i32 = 0;
    let mut a: [i32; 3] = [0, 1, 2];
    'loop_: while ((x) < (3)) {
        a[(x.postfix_inc()) as usize].prefix_inc();
    }
    let mut out: i32 = 0;
    'loop_: while (x != 0) {
        out += a[(x.prefix_dec()) as usize];
    }
    out.postfix_inc();
    let mut x2: i32 = out.prefix_dec();
    out.prefix_inc();
    let mut x3: i32 = out.postfix_dec();
    return (((out.postfix_inc()) + (x2)) + (x3));
}
