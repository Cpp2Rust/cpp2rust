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
    let mut arr: [u8; 8] = [120_u8, 86_u8, 52_u8, 18_u8, 239_u8, 205_u8, 171_u8, 144_u8];
    let mut dwords: *mut u32 = (arr.as_mut_ptr() as *mut u32);
    assert!(((*dwords.offset((0) as isize)) == (305419896_u32)));
    assert!(((*dwords.offset((1) as isize)) == (2427178479)));
    return 0;
}
