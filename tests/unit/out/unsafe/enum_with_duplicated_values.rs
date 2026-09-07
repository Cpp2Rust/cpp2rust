extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type E = u32;
pub const E_A: E = 2;
pub const E_B: E = 4;
pub const E_C: E = 2;
pub const E_D: E = 4;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((E_A as i32) == ((1) << (1))));
    assert!(((E_B as i32) == ((1) << (2))));
    assert!(((E_C as i32) == ((1) << (1))));
    assert!(((E_D as i32) == ((1) << (2))));
    return 0;
}
