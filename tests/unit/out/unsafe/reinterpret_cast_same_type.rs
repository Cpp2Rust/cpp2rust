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
    let mut arr: [u8; 4] = [10_u8, 20_u8, 30_u8, 40_u8];
    let mut same: *mut u8 = arr.as_mut_ptr();
    assert!((((*same.offset((0) as isize)) as i32) == (10)));
    assert!((((*same.offset((1) as isize)) as i32) == (20)));
    assert!((((*same.offset((2) as isize)) as i32) == (30)));
    assert!((((*same.offset((3) as isize)) as i32) == (40)));
    (*same.offset((2) as isize)) = 99_u8;
    assert!(((arr[(2) as usize] as i32) == (99)));
    return 0;
}
