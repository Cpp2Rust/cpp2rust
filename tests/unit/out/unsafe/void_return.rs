extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f1_0(mut first: *mut i32, mut last: *mut i32) {
    {
        let len = last.offset_from(first) as usize;
        ::std::slice::from_raw_parts_mut(first, len).sort()
    };
    return;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = vec![1, 3, 0, 2, 8, 7];
    (unsafe {
        let _first: *mut i32 = v.as_mut_ptr();
        let _last: *mut i32 = v.as_mut_ptr().add(v.len());
        f1_0(_first, _last)
    });
    return 0;
}
