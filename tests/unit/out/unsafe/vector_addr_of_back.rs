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
    let mut outer: Vec<Vec<i32>> = Vec::new();
    let mut inner: Vec<i32> = Vec::new();
    outer.push(inner.clone());
    let mut sink: *mut Vec<i32> = ((outer).last_mut().unwrap());
    assert!((((*sink.cast_const()).len() as u64) == (0_u64)));
    let mut p: *mut Vec<Vec<i32>> = (&mut outer as *mut Vec<Vec<i32>>);
    sink = ((*p).last_mut().unwrap());
    assert!((((*sink.cast_const()).len() as u64) == (0_u64)));
    return 0;
}
