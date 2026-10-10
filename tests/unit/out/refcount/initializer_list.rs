extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(bytes: Vec<i32>) -> usize {
    let bytes: Value<Vec<i32>> = Rc::new(RefCell::new(bytes));
    let mut buf: Ptr<Vec<i32>> = Ptr::alloc((*bytes.borrow()).clone());
    let mut n: usize = (*bytes.borrow()).len();
    buf.delete();
    return n;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ f_0(vec![1, 2, 3,],) }) == 3_usize));
    let mut v: Vec<i32> = vec![4, 5, 6];
    assert!((v.len() == 3_usize));
    assert!((((v[0_usize] + v[1_usize]) + v[2_usize]) == 15));
    let l: Value<Vec<i32>> = Rc::new(RefCell::new(vec![7, 8]));
    assert!(((*l.borrow()).len() == 2_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
