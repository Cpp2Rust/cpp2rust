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
    let mut storage: i32 = 7;
    let mut p: *mut i32 = (&mut storage as *mut i32);
    let mut np: *mut i32 = Default::default();
    if !(p).is_null() {
        assert!(true);
    }
    if !(!(p).is_null()) {
        assert!(false);
    }
    if !(np).is_null() {
        assert!(false);
    }
    if !(!(np).is_null()) {
        assert!(true);
    }
    let mut iter: *mut i32 = p;
    let mut iters: i32 = 0;
    'loop_: while !(iter).is_null() {
        iters.prefix_inc();
        iter = Default::default();
    }
    assert!(((iters) == (1)));
    let mut t3: i32 = if !(p).is_null() { 1 } else { 0 };
    assert!(((t3) == (1)));
    let mut t4: i32 = if !(np).is_null() { 1 } else { 0 };
    assert!(((t4) == (0)));
    let mut t5: i32 = (!(!(p).is_null()) as i32);
    assert!(((t5) == (0)));
    let mut t6: i32 = (!(!(np).is_null()) as i32);
    assert!(((t6) == (1)));
    let mut b2: bool = !(p).is_null();
    let mut b3: bool = !(np).is_null();
    assert!(b2);
    assert!(!(b3));
    return 0;
}
