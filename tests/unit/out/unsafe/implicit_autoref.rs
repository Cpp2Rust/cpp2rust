extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, Default)]
pub struct Holder {
    pub v: Vec<i32>,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    let mut p: *mut Vec<i32> = (&mut v as *mut Vec<i32>);
    let mut a: i32 = (&mut (*p))[(0_u64) as usize];
    (&mut (*p))[(1_u64) as usize] = 30;
    let mut h: Holder = <Holder>::default();
    h.v.push(40);
    h.v.push(50);
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    let mut b: i32 = (&mut (*hp)).v[(0_u64) as usize];
    (&mut (*hp)).v[(1_u64) as usize] = 60;
    assert!(((a) == (10)));
    assert!((((&mut (*p))[(1_u64) as usize]) == (30)));
    assert!(((b) == (40)));
    assert!((((&mut (*hp)).v[(1_u64) as usize]) == (60)));
    return 0;
}
