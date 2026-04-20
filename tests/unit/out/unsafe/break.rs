extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn for_test_0(n: i32) -> i32 {
    let mut x: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    'loop_: while ((i) < (n)) {
        x += 1;
        if ((x) == (100)) {
            break;
        }
        let mut k: i32 = 0;
        let mut w: i32 = 0;
        'loop_: while ((w) < (j)) {
            break;
            w += 1;
            k += 1;
            i += k;
        }
        x = ((x) + (1));
        j = i;
        i += 1;
    }
    return x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return (unsafe {
        let _n: i32 = 200;
        for_test_0(_n)
    });
}
