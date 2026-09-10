extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C, align(16))]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub c: libc::c_char,
    pub x: i64,
}
pub unsafe fn pack_size_0() -> u64 {
    return ((0 as usize).wrapping_add((0 as usize)) as u64);
}
pub unsafe fn pack_size_1(mut args_0: i32, mut args_1: f64) -> u64 {
    return ((2 as usize).wrapping_add((2 as usize)) as u64);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [i64; 4] = [0_i64, 0_i64, 0_i64, 0_i64];
    let mut s: S = S {
        c: (0 as libc::c_char),
        x: 0_i64,
    };
    assert!(((::std::mem::size_of::<i32>()) == (4_usize)));
    assert!(((::std::mem::size_of::<[i64; 4]>()) == (32_usize)));
    assert!(((::std::mem::size_of::<S>()) == (16_usize)));
    assert!(((::std::mem::align_of::<i32>()) == (4_usize)));
    assert!(((::std::mem::align_of::<S>()) == (16_usize)));
    assert!(((::std::mem::align_of::<[i64; 4]>()) == (8_usize)));
    assert!(((::std::mem::align_of::<S>()) == (16_usize)));
    assert!(((unsafe { pack_size_0() }) == (0_u64)));
    assert!(((unsafe { pack_size_1(1, 2.0E+0,) }) == (4_u64)));
    return 0;
}
