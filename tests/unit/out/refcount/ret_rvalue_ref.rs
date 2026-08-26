extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(v: i32) -> i32 {
    return (v.read());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i2: Value<i32> = 5;
    assert!(((i2.read()) == 5));
    let i3: Value<i32> = Rc::new(RefCell::new(({ foo_0((i2.read())) })));
    assert!(((*i3.borrow()) == 5));
    assert!((({ foo_0(5,) }) == 5));
    return 0;
}
