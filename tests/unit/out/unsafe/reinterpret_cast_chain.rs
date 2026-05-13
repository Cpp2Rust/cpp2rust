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
    let mut val: u64 = 578437695752307201_u64;
    let mut dwords: *mut u32 = ((&mut val as *mut u64) as *mut u32);
    assert!(((*dwords.offset((0) as isize)) == (67305985_u32)));
    assert!(((*dwords.offset((1) as isize)) == (134678021_u32)));
    let mut words: *mut u16 = (dwords as *mut u16);
    assert!((((*words.offset((0) as isize)) as i32) == (513)));
    assert!((((*words.offset((1) as isize)) as i32) == (1027)));
    assert!((((*words.offset((2) as isize)) as i32) == (1541)));
    assert!((((*words.offset((3) as isize)) as i32) == (2055)));
    (*words.offset((1) as isize)) = 48042_u16;
    assert!(((*dwords.offset((0) as isize)) == (3148481025)));
    assert!(((val) == (578437698833482241)));
    (*dwords.offset((1) as isize)) = 4293844428;
    assert!(((val) == (18441921395520307713)));
    assert!((((*words.offset((2) as isize)) as i32) == (56780)));
    assert!((((*words.offset((3) as isize)) as i32) == (65518)));
    return 0;
}
