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
    let flag: Value<u32> = Rc::new(RefCell::new(7_u32));
    let b1: Value<bool> = Rc::new(RefCell::new(((*flag.borrow()) != 0)));
    let b2: Value<bool> = Rc::new(RefCell::new((0 != 0)));
    assert!((*b1.borrow()));
    assert!(!(*b2.borrow()));
    return 0;
}
