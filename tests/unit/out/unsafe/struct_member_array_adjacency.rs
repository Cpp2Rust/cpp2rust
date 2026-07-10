extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pair {
    pub a: [i32; 4],
    pub b: [i32; 4],
}
impl Default for pair {
    fn default() -> Self {
        pair {
            a: [0_i32; 4],
            b: [0_i32; 4],
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: pair = <pair>::default();
    assert!(((((s.a.as_mut_ptr().offset((4) as isize)) == (s.b.as_mut_ptr())) as i32) != 0));
    return 0;
}
