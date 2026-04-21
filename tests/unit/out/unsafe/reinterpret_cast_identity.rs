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
    let mut val: u32 = 42_u32;
    let mut original: *mut u32 = (&mut val as *mut u32);
    let mut as_u16: *mut u16 = (original as *mut u16);
    let mut back: *mut u32 = (as_u16 as *mut u32);
    assert!(((back) == (original)));
    assert!(((*back) == (42_u32)));
    let mut as_u8: *mut u8 = (original as *mut u8);
    let mut back2: *mut u32 = (as_u8 as *mut u32);
    assert!(((back2) == (original)));
    assert!(((*back2) == (42_u32)));
    return 0;
}
