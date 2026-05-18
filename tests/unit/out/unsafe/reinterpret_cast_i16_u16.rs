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
    let mut vals: [i16; 3] = [(-1_i32 as i16), 256_i16, (-32768_i32 as i16)];
    let mut uvals: *mut u16 = (vals.as_mut_ptr() as *mut u16);
    assert!((((*uvals.offset((0) as isize)) as i32) == (65535)));
    assert!((((*uvals.offset((1) as isize)) as i32) == (256)));
    assert!((((*uvals.offset((2) as isize)) as i32) == (32768)));
    (*uvals.offset((0) as isize)) = 42_u16;
    assert!(((vals[(0) as usize] as i32) == (42)));
    return 0;
}
