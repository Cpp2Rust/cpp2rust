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
    assert!((((1 == 1) as i32) != 0));
    assert!((((0 == 0) as i32) != 0));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    assert!((((1 == 1) as i32) != 0));
    assert!((((0 == 0) as i32) != 0));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    assert!((((1 == 1) as i32) != 0));
    assert!((((0 == 0) as i32) != 0));
    let ul: Value<u64> = Rc::new(RefCell::new(0_u64));
    assert!((((1 == 1) as i32) != 0));
    assert!((((0 == 0) as i32) != 0));
    return 0;
}
