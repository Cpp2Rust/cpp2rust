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
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let mut p1: Ptr<i32> = (x.as_pointer());
    let mut p2: Ptr<i32> = (x.as_pointer());
    assert!(!({ (p1).clone() } != { (p2).clone() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
