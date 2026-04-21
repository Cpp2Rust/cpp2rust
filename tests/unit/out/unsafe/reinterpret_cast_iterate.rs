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
    let mut arr: [u64; 2] = [1125912791875585_u64, 2251829878849541_u64];
    let mut words: *mut u16 = (arr.as_mut_ptr() as *mut u16);
    let mut i: i32 = 0;
    'loop_: while ((i) < (8)) {
        assert!((((*words.offset((i) as isize)) as i32) == (((((i) + (1)) as i32) as u16) as i32)));
        i.postfix_inc();
    }
    return 0;
}
