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
    let mut x: i32 = 1;
    let mut y: i32 = {
        x = 2;
        ((x) + (1))
    };
    assert!(((x) == (2)));
    assert!(((y) == (3)));
    let mut z: i32 = {
        1;
        2;
        3
    };
    assert!(((z) == (3)));
    let mut counter: i32 = 0;
    let mut w: i32 = {
        counter.postfix_inc();
        counter.postfix_inc();
        counter
    };
    assert!(((counter) == (2)));
    assert!(((w) == (2)));
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    if {
        a = 1;
        b = 2;
        (((a) + (b)) > (0))
    } {
        assert!(((a) == (1)));
        assert!(((b) == (2)));
    }
    return 0;
}
