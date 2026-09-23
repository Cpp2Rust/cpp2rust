extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type E = u32;
pub const E_A: E = 0;
pub const E_B: E = 1;
pub const E_C: E = 2;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut size: usize = 1_usize;
    assert!(((size) == (1_usize)));
    let mut ul: u64 = 5_u64;
    let mut s1: usize = (ul as usize);
    assert!(((s1) == (5_usize)));
    ul = (7_usize as u64);
    assert!(((ul) == (7_u64)));
    let mut i: i32 = 2;
    let mut e: E = ((i) as E);
    assert!(((e as i32) == (E_C as i32)));
    assert!(((e as i32) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
