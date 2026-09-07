extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut c: Color = ((n) as Color);
    return if ((c as i32) == (Color_BLUE as i32)) {
        0
    } else {
        1
    };
}
