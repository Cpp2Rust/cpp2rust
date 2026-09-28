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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v1: Vec<Vec<i32>> = Vec::new();
    let mut v2: Vec<i32> = vec![1];
    v1.push(v2.clone());
    assert!(((*&mut (*((v1).last_mut().unwrap()))[0_usize as usize]) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
