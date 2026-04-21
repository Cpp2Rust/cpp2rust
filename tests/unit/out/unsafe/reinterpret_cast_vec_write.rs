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
    let mut vec_: Vec<u32> = Vec::new();
    vec_.push(67305985_u32);
    vec_.push(134678021_u32);
    let mut bytes: *mut u8 = (vec_.as_mut_ptr() as *mut u8);
    assert!((((*bytes.offset((0) as isize)) as i32) == (1)));
    assert!((((*bytes.offset((1) as isize)) as i32) == (2)));
    assert!((((*bytes.offset((2) as isize)) as i32) == (3)));
    assert!((((*bytes.offset((3) as isize)) as i32) == (4)));
    assert!((((*bytes.offset((4) as isize)) as i32) == (5)));
    assert!((((*bytes.offset((7) as isize)) as i32) == (8)));
    (*bytes.offset((4) as isize)) = 255_u8;
    assert!(((vec_[(1_u64) as usize]) == (134678271_u32)));
    return 0;
}
