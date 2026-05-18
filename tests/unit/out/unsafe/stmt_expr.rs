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
    let mut x: i32 = {
        let mut a: i32 = 1;
        let mut b: i32 = 2;
        ((a) + (b))
    };
    assert!(((x) == (3)));
    let mut counter: i32 = 0;
    let mut y: i32 = {
        counter.postfix_inc();
        ((counter) * (10))
    };
    assert!(((y) == (10)));
    assert!(((counter) == (1)));
    let mut z: i32 = {
        let mut v: i32 = 5;
        if ((v) > (0)) {
            v = ((v) * (2));
        }
        v
    };
    assert!(((z) == (10)));
    assert!(
        (({
            let mut inner: i32 = {
                let mut a: i32 = 100;
                a
            };
            inner
        }) == (100))
    );
    return 0;
}
