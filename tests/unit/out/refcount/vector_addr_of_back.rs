extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let outer: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    let inner: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (outer.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new((*inner.borrow()).clone())))
    });
    let mut sink: Ptr<Vec<i32>> = ((*outer.borrow())[(*outer.borrow()).len() - 1].as_pointer());
    assert!(((*sink.upgrade().deref()).len() == 0_usize));
    let mut p: Ptr<Vec<Value<Vec<i32>>>> = (outer.as_pointer());
    sink = ((*p.upgrade().deref())[(*p.upgrade().deref()).len() - 1].as_pointer());
    assert!(((*sink.upgrade().deref()).len() == 0_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
