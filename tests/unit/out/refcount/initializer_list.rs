extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(bytes: Vec<i32>) -> u64 {
    let bytes: Value<Vec<i32>> = Rc::new(RefCell::new(bytes));
    let buf: Value<Ptr<Vec<i32>>> = Rc::new(RefCell::new(Ptr::alloc((*bytes.borrow()).clone())));
    let n: Value<u64> = Rc::new(RefCell::new((*bytes.borrow()).len() as u64));
    (*buf.borrow()).delete();
    return (*n.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _bytes: Vec<i32> = vec![1, 2, 3];
            f_0(_bytes)
        }) == 3_u64)
    );
    return 0;
}
