extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn escape_0(ptr: Ptr<i32>) {
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(ptr));
    (*ptr.borrow()).delete();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let alloc: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(1)));
    ({
        let _ptr: Ptr<i32> = (*alloc.borrow()).clone();
        escape_0(_ptr)
    });
    (*alloc.borrow()).delete();
    return 0;
}
