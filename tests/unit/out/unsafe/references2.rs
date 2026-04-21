extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn change_0(p: *mut Option<Box<i32>>) {
    let mut q: Option<Box<i32>> = Some(Box::new(7));
    (*p) = q;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Option<Box<i32>> = Some(Box::new(5));
    (unsafe {
        let _p: *mut Option<Box<i32>> = &mut a as *mut Option<Box<i32>>;
        change_0(_p)
    });
    return (*a.as_deref_mut().unwrap());
}
