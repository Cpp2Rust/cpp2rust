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
    let mut p: *mut u32 = (Box::leak(Box::new(67305985_u32)) as *mut u32);
    let mut bytes: *mut u8 = (p as *mut u8);
    assert!((((*bytes.offset((0) as isize)) as i32) == (1)));
    assert!((((*bytes.offset((1) as isize)) as i32) == (2)));
    assert!((((*bytes.offset((2) as isize)) as i32) == (3)));
    assert!((((*bytes.offset((3) as isize)) as i32) == (4)));
    (*bytes.offset((0) as isize)) = 16_u8;
    assert!(((*p) == (67306000_u32)));
    ::std::mem::drop(Box::from_raw(p));
    return 0;
}
