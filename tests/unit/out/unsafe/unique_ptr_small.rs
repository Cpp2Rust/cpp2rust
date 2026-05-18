extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn change_0(n: *mut Option<Box<i32>>) {
    let mut m: Option<Box<i32>> = Some(Box::new(20));
    (*n) = m;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: Option<Box<i32>> = Some(Box::new(10));
    (unsafe {
        let _n: *mut Option<Box<i32>> = &mut n as *mut Option<Box<i32>>;
        change_0(_n)
    });
    return (*n.as_deref_mut().unwrap());
}
