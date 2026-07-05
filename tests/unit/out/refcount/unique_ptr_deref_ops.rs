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
    let p: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(10)))));
    (*(*p.borrow_mut()).as_ref().unwrap().borrow_mut()) += 5;
    (*(*p.borrow_mut()).as_ref().unwrap().borrow_mut()) -= 3;
    (*(*p.borrow_mut()).as_ref().unwrap().borrow_mut()) *= 2;
    let q: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(1)))));
    let sum: Value<i32> = Rc::new(RefCell::new(
        ((*(*p.borrow()).as_ref().unwrap().borrow()) + (*(*q.borrow()).as_ref().unwrap().borrow())),
    ));
    return (*sum.borrow());
}
