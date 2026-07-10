extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(array: Ptr<i32>) {
    let array: Value<Ptr<i32>> = Rc::new(RefCell::new(array));
    (*array.borrow()).delete_array();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(1)));
    ({ foo_0((*x.borrow()).clone()) });
    return 0;
}
