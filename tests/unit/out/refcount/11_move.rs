extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn change_0(n: Ptr<Option<Value<i32>>>) {
    let m: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(20)))));
    let __rhs = (*m.borrow_mut()).take();
    n.write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(10)))));
    ({
        let _n: Ptr<Option<Value<i32>>> = n.as_pointer();
        change_0(_n)
    });
    return (*(*n.borrow()).as_ref().unwrap().borrow());
}
