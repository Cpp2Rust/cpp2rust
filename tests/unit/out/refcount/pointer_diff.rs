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
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
    let p0: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((a.as_pointer() as Ptr<i32>).offset(0 as isize)),
    ));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((a.as_pointer() as Ptr<i32>).offset(4 as isize)),
    ));
    return ((((*p1.borrow()).clone() - (*p0.borrow()).clone()) as i64) as i32);
}
