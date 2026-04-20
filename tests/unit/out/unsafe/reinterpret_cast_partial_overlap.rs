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
    let mut arr: [u32; 2] = [67305985_u32, 134678021_u32];
    let mut bytes: *mut u8 = (arr.as_mut_ptr() as *mut u8);
    assert!((((*bytes.offset((3) as isize)) as i32) == (4)));
    assert!((((*bytes.offset((4) as isize)) as i32) == (5)));
    (*bytes.offset((3) as isize)) = 255_u8;
    assert!(((arr[(0) as usize]) == (4278387201_u32)));
    (*bytes.offset((4) as isize)) = 170_u8;
    assert!(((arr[(1) as usize]) == (134678186_u32)));
    return 0;
}
