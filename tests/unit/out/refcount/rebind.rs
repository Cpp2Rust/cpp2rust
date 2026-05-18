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
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let r: Ptr<i32> = x.as_pointer();
    let y: Value<i32> = Rc::new(RefCell::new(10));
    let __rhs = (*y.borrow());
    r.write(__rhs);
    (*y.borrow_mut()) += 1;
    return (*x.borrow());
}
