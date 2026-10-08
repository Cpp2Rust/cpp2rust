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
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let mut p0: Ptr<i32> = ((a.as_pointer() as Ptr<i32>).offset(0));
    let mut p1: Ptr<i32> = ((a.as_pointer() as Ptr<i32>).offset(4));
    assert!((((p1).clone() - (p0).clone()) as i64 == 4_i64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
