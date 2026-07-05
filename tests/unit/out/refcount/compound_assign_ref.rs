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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(10);
    let rhs_0 = ((((v.as_pointer() as Ptr<i32>).read()) as i32) + 5) as i32;
    (v.as_pointer() as Ptr<i32>).write(rhs_0);
    assert!((((v.as_pointer() as Ptr<i32>).read()) == 15));
    return ((v.as_pointer() as Ptr<i32>).read());
}
