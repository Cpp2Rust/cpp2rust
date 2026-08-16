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
    let mut b1: Vec<bool> = (0..u64::BITS).map(|i| (5_u64 >> i) & 1 == 1).collect();
    let mut b2: Vec<bool> = {
        let mut bits = vec![false; 2_usize];
        bits.extend(b1.iter());
        bits
    };
    let mut b3: Vec<bool> = b2.iter().skip(1_usize).copied().collect();
    assert!(((b1.iter().rev().fold(0u64, |acc, &b| (acc << 1) | b as u64)) == (5_u64)));
    assert!(((b2.iter().rev().fold(0u64, |acc, &b| (acc << 1) | b as u64)) == (20_u64)));
    assert!(((b3.iter().rev().fold(0u64, |acc, &b| (acc << 1) | b as u64)) == (10_u64)));
    return 0;
}
