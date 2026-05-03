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
    let xu8: u8 = 8_u8;
    let xu16: u16 = 16_u16;
    let mut xu32: u32 = 32_u32;
    let mut xu64: u64 = 64_u64;
    let xsz1: u64 = 64_u64;
    let mut xsz2: u64 = 64_u64;
    let xi1: i8 = (-8_i32 as i8);
    let xi2: i16 = 16_i16;
    let mut xi3: i32 = 32;
    let mut xi4: i64 = 64_i64;
    let mut b: bool = ((xu64) == (64_u64));
    return (((((((((((((xu8 as i32) + (xu16 as i32)) as u32).wrapping_add(xu32)) as u64)
        .wrapping_add(xu64))
    .wrapping_add(xsz1))
    .wrapping_add(xsz2))
    .wrapping_add((xi1 as u64)))
    .wrapping_add((xi2 as u64)))
    .wrapping_add((xi3 as u64)))
    .wrapping_add((xi4 as u64))) as i32);
}
