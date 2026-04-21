extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn null_0() -> Ptr<i32> {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(Default::default()));
    return (*p.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(({ null_0() })));
    return ((*x.borrow()).read());
}
