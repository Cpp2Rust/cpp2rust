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
    let mut view1: *mut u32 = ((&mut val as *mut u64) as *mut u32);
    let mut view2: *mut u32 = ((&mut val as *mut u64) as *mut u32);
    (*view1.offset((0) as isize)) = 3721182122;
    assert!(((*view2.offset((0) as isize)) == (3721182122)));
    assert!(((val) == (578437699406183338)));
    (*view2.offset((1) as isize)) = 4293844428;
    assert!(((*view1.offset((1) as isize)) == (4293844428)));
    assert!(((val) == (18441921396093008810)));
    return 0;
}
