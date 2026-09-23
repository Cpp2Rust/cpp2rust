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
    let buf: Value<Ptr<Vec<i32>>> = Rc::new(RefCell::new(Ptr::alloc((*bytes.borrow()).clone())));
    let n: Value<usize> = Rc::new(RefCell::new((*bytes.borrow()).len()));
    (*buf.borrow()).delete();
    return (*n.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ f_0(vec![1, 2, 3,],) }) == 3_usize));
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(vec![4, 5, 6]));
    assert!(((*v.borrow()).len() == 3_usize));
    assert!(
        (((((v.as_pointer() as Ptr<i32>).offset(0_usize).read())
            + ((v.as_pointer() as Ptr<i32>).offset(1_usize).read()))
            + ((v.as_pointer() as Ptr<i32>).offset(2_usize).read()))
            == 15)
    );
    let l: Value<Vec<i32>> = Rc::new(RefCell::new(vec![7, 8]));
    assert!(((*l.borrow()).len() == 2_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
