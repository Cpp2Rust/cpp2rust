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
    let p1: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(2.try_into().expect("failed conversion"))),
    )));
    assert!(((*(*p1.borrow()).0.borrow()) == 1.0E+0));
    assert!(((*(*p1.borrow()).1.borrow()) == 2));
    let p2: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(3.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(4.try_into().expect("failed conversion"))),
    )));
    assert!(((*(*p2.borrow()).0.borrow()) == 3.0E+0));
    assert!(((*(*p2.borrow()).1.borrow()) == 4));
    let p3: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(5.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(6.try_into().expect("failed conversion"))),
    )));
    assert!(((*(*p3.borrow()).0.borrow()) == 5.0E+0));
    assert!(((*(*p3.borrow()).1.borrow()) == 6));
    return 0;
}
