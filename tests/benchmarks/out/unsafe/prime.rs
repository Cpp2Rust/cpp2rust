extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_prime_0(mut x: i32) -> bool {
    let mut i: i32 = 2;
    'loop_: while ((i) < (x)) {
        if (((x) % (i)) == (0)) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
pub unsafe fn largest_prime_1(mut n: i32) -> i32 {
    let mut max: i32 = -1_i32;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        if (unsafe {
            let _x: i32 = i;
            is_prime_0(_x)
        }) {
            max = i;
        }
        i.prefix_inc();
    }
    return max;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut N: i32 = 270000;
    let mut largest: i32 = (unsafe {
        let _n: i32 = N;
        largest_prime_1(_n)
    });
    return largest;
}
