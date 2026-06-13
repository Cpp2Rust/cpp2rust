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
    let mut s: Vec<::core::ffi::c_char> = {
        let s = c"ABCD".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let mut bytes: *const u8 = (s.as_ptr() as *const u8);
    assert!((((*bytes.offset((0) as isize)) as i32) == (('A' as ::core::ffi::c_char) as i32)));
    assert!((((*bytes.offset((1) as isize)) as i32) == (('B' as ::core::ffi::c_char) as i32)));
    assert!((((*bytes.offset((2) as isize)) as i32) == (('C' as ::core::ffi::c_char) as i32)));
    assert!((((*bytes.offset((3) as isize)) as i32) == (('D' as ::core::ffi::c_char) as i32)));
    assert!((((*bytes.offset((4) as isize)) as i32) == (0)));
    return 0;
}
