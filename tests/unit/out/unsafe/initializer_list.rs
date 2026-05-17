extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut bytes: Vec<i32>) -> u64 {
    let mut buf: *mut Vec<i32> = (Box::leak(Box::new(bytes)) as *mut Vec<i32>);
    let mut n: u64 = bytes.len() as u64;
    ::std::mem::drop(Box::from_raw(buf));
    return n;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _bytes: Vec<i32> = vec![1, 2, 3];
            f_0(_bytes)
        }) == (3_u64))
    );
    return 0;
}
