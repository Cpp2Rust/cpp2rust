extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn classify_0(x: Ptr<i32>) -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    {
        return ((*x.borrow()).read());
    }
    return 1;
}
pub fn classify_1(x: i64) -> i32 {
    let x: Value<i64> = Rc::new(RefCell::new(x));
    {
        return 2;
    }
    return 1;
}
pub fn classify_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return 1;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(7));
    assert!((({ classify_0((v.as_pointer()),) }) == 7));
    assert!((({ classify_1(1_i64,) }) == 2));
    assert!((({ classify_2(1,) }) == 1));
    return 0;
}
