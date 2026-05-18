extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fib_0(mut n: *mut u64) {
    if ((*n) == (0_u64)) || ((*n) == (1_u64)) {
        return;
    }
    let mut n_1: u64 = (*n).wrapping_sub(1_u64);
    let mut n_2: u64 = (*n).wrapping_sub(2_u64);
    (unsafe {
        let _n: *mut u64 = (&mut n_1 as *mut u64);
        fib_0(_n)
    });
    (unsafe {
        let _n: *mut u64 = (&mut n_2 as *mut u64);
        fib_0(_n)
    });
    (*n) = (n_1).wrapping_add(n_2);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: u64 = 45_u64;
    (unsafe {
        let _n: *mut u64 = (&mut n as *mut u64);
        fib_0(_n)
    });
    return (n as i32);
}
