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
    let mut value: u32 = 67305985_u32;
    let mut bytes: *mut u8 = ((&mut value as *mut u32) as *mut u8);
    assert!((((*bytes.offset((0) as isize)) as i32) == (1)));
    assert!((((*bytes.offset((1) as isize)) as i32) == (2)));
    assert!((((*bytes.offset((2) as isize)) as i32) == (3)));
    assert!((((*bytes.offset((3) as isize)) as i32) == (4)));
    let mut arr: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let mut arr16: *mut u16 = (arr.as_mut_ptr() as *mut u16);
    assert!((((*arr16.offset((0) as isize)) as i32) == (513)));
    assert!((((*arr16.offset((1) as isize)) as i32) == (1027)));
    return 0;
}
