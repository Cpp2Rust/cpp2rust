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
pub static mut global_0: i32 = unsafe { (((E_A as i32) != (E_B as i32)) as i32) };
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((global_0) == (1)));
    return 0;
}
