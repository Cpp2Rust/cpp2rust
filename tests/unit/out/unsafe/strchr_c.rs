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
    let mut s: *const u8 = b"hello world\0".as_ptr().cast_mut().cast_const();
    let mut r: *mut u8 = libc::strchr(s as *const i8, ('w' as i32)) as *mut u8;
    assert!((((!((r).is_null())) as i32) != 0));
    assert!((((((*r) as i32) == ('w' as i32)) as i32) != 0));
    assert!(((((libc::strchr(s as *const i8, ('z' as i32)) as *mut u8).is_null()) as i32) != 0));
    return 0;
}
