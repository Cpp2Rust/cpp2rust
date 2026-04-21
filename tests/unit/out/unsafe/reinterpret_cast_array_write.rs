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
    let mut arr: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let mut words: *mut u16 = (arr.as_mut_ptr() as *mut u16);
    assert!((((*words.offset((0) as isize)) as i32) == (513)));
    assert!((((*words.offset((1) as isize)) as i32) == (1027)));
    (*words.offset((0) as isize)) = 48042_u16;
    assert!(((arr[(0) as usize] as i32) == (170)));
    assert!(((arr[(1) as usize] as i32) == (187)));
    assert!(((arr[(2) as usize] as i32) == (3)));
    assert!(((arr[(3) as usize] as i32) == (4)));
    words = (arr.as_mut_ptr().offset((1) as isize) as *mut u16);
    assert!((((*words.offset((0) as isize)) as i32) == (955)));
    (*words.offset((0) as isize)) = 0_u16;
    assert!(((arr[(0) as usize] as i32) == (170)));
    assert!(((arr[(1) as usize] as i32) == (0)));
    assert!(((arr[(2) as usize] as i32) == (0)));
    assert!(((arr[(3) as usize] as i32) == (4)));
    return 0;
}
