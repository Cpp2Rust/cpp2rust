extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone)]
pub struct Handler {
    pub tag: i32,
    pub cb: Option<unsafe fn(i32) -> i32>,
}
impl Default for Handler {
    fn default() -> Self {
        Handler {
            tag: 0 as i32,
            cb: None,
        }
    }
}
pub unsafe fn double_it_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn negate_1(mut x: i32) -> i32 {
    return -x;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h1: Handler = Handler {
        tag: 1,
        cb: Some(double_it_0 as _),
    };
    let mut h2: Handler = Handler {
        tag: 2,
        cb: Some(negate_1 as _),
    };
    assert!(!((h1.cb).is_none()));
    assert!(
        ((unsafe {
            let _arg0: i32 = 5;
            (h1.cb).unwrap()(_arg0)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _arg0: i32 = 7;
            (h2.cb).unwrap()(_arg0)
        }) == (-7_i32))
    );
    (h1.cb) = Some(negate_1 as _);
    assert!(
        ((unsafe {
            let _arg0: i32 = 3;
            (h1.cb).unwrap()(_arg0)
        }) == (-3_i32))
    );
    assert!(((h1.cb) == (h2.cb)));
    return 0;
}
