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
    let rhs_0 = (((*(*p.borrow()).as_ref().unwrap().borrow()) as i32) + 5) as i32;
    (*(*p.borrow_mut()).as_ref().unwrap().borrow_mut()) = rhs_0;
    let rhs_0 = (((*(*p.borrow()).as_ref().unwrap().borrow()) as i32) - 3) as i32;
    (*(*p.borrow_mut()).as_ref().unwrap().borrow_mut()) = rhs_0;
    let rhs_0 = (((*(*p.borrow()).as_ref().unwrap().borrow()) as i32) * 2) as i32;
    (*(*p.borrow_mut()).as_ref().unwrap().borrow_mut()) = rhs_0;
    let q: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(1)))));
    let sum: Value<i32> = Rc::new(RefCell::new(
        ((*(*p.borrow()).as_ref().unwrap().borrow()) + (*(*q.borrow()).as_ref().unwrap().borrow())),
    ));
    return (*sum.borrow());
}
