extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn escape_0(p: Ptr<i32>) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    (*p.borrow()).delete();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(1)));
    ({
        let _p: Ptr<i32> = (*p1.borrow()).clone();
        escape_0(_p)
    });
    return ((*p1.borrow()).read());
}
