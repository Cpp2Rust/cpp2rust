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
    let mut neg: i32 = -1_i32;
    let mut words: *mut u16 = ((&mut neg as *mut i32) as *mut u16);
    assert!((((*words.offset((0) as isize)) as i32) == (65535)));
    assert!((((*words.offset((1) as isize)) as i32) == (65535)));
    let mut neg64: i64 = (-256_i32 as i64);
    let mut quarters: *mut i16 = ((&mut neg64 as *mut i64) as *mut i16);
    assert!((((*quarters.offset((0) as isize)) as i32) == (-256_i32)));
    assert!((((*quarters.offset((1) as isize)) as i32) == (-1_i32)));
    assert!((((*quarters.offset((2) as isize)) as i32) == (-1_i32)));
    assert!((((*quarters.offset((3) as isize)) as i32) == (-1_i32)));
    return 0;
}
