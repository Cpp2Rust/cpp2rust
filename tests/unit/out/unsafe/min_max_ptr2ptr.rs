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
    let mut a: i32 = 10;
    let mut b: i32 = 20;
    let mut pa: *mut i32 = (&mut a as *mut i32);
    let mut pb: *mut i32 = (&mut b as *mut i32);
    let mut ppa: *mut *mut i32 = (&mut pa as *mut *mut i32);
    let mut ppb: *mut *mut i32 = (&mut pb as *mut *mut i32);
    let mut r1: i32 = (*if *(*ppa) >= *(*ppb) {
        (*ppa) as *const _
    } else {
        (*ppb) as *const _
    });
    let mut r2: i32 = (*if *(*ppa) <= *(*ppb) {
        (*ppa) as *const _
    } else {
        (*ppb) as *const _
    });
    return ((r1) + (r2));
}
