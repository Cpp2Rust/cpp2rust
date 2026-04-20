extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[Value<Box<[i32]>>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(Box::new([1, 2]))),
        Rc::new(RefCell::new(Box::new([3, 4]))),
    ])));
    return 0;
}
