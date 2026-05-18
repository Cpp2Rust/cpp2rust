extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn change_0(p: Ptr<Option<Value<i32>>>) {
    let q: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(7)))));
    let __rhs = (*q.borrow_mut()).take();
    p.write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(5)))));
    ({
        let _p: Ptr<Option<Value<i32>>> = a.as_pointer();
        change_0(_p)
    });
    return (*(*a.borrow()).as_ref().unwrap().borrow());
}
