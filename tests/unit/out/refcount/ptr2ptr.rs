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
    let out: Value<i32> = Rc::new(RefCell::new(0));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    (*p1.borrow()).write(1);
    (*out.borrow_mut()) *= (*x.borrow());
    let p2: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new((p1.as_pointer())));
    ((*p2.borrow()).read()).write(2);
    (*out.borrow_mut()) *= (*x.borrow());
    let p3: Value<Ptr<Ptr<Ptr<i32>>>> = Rc::new(RefCell::new((p2.as_pointer())));
    (((*p3.borrow()).read()).read()).write(3);
    (*out.borrow_mut()) *= (*x.borrow());
    return (*out.borrow());
}
