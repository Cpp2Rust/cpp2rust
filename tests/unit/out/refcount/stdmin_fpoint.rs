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
    let a0: Value<f64> = Rc::new(RefCell::new(3.14E+0));
    let a1: Value<f64> = Rc::new(RefCell::new(2.71E+0));
    if a0.as_pointer().read() <= a1.as_pointer().read() {
        a0.as_pointer()
    } else {
        a1.as_pointer()
    };
    return 0;
}
