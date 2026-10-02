extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let out: Value<i32> = Rc::new(RefCell::new(0));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 0])));
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<i32>).offset(0))));
    'loop_: while (((*ptr.borrow()).read()) != 0) {
        (*out.borrow_mut()) += { ((*ptr.borrow()).read()) };
        (*ptr.borrow_mut()).prefix_inc();
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<i32>).offset(1))));
    'loop_: while (((*ptr.borrow()).read()) != 4) {
        (*out.borrow_mut()) += { ((*ptr.borrow()).read()) };
        (*ptr.borrow_mut()).postfix_inc();
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<i32>).offset(4))));
    'loop_: while (((*ptr.borrow()).read()) != 1) {
        (*out.borrow_mut()) += { ((*ptr.borrow()).read()) };
        (*ptr.borrow_mut()).postfix_dec();
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<i32>).offset(3))));
    'loop_: while (((*ptr.borrow()).read()) != 2) {
        (*out.borrow_mut()) += { ((*ptr.borrow()).read()) };
        (*ptr.borrow_mut()).prefix_dec();
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<i32>).offset(0))));
    'loop_: while (((*ptr.borrow()).read()) != 0) {
        (*out.borrow_mut()) += { ((*ptr.borrow()).read()) };
        (*ptr.borrow_mut()) = { (*ptr.borrow()).offset((1) as isize) };
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<i32>).offset(0))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 5) {
        (*out.borrow_mut()) += { (elem!((*ptr.borrow()), (*i.borrow())).read()) };
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(((*out.borrow()) == 51));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
