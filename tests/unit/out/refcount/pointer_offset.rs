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
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 0])));
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<i32>).offset(0 as isize)),
    ));
    'loop_: while (((*ptr.borrow()).read()) != 0) {
        let __rhs = ((*ptr.borrow()).read());
        (*out.borrow_mut()) += __rhs;
        (*ptr.borrow_mut()).prefix_inc();
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<i32>).offset(1 as isize)),
    ));
    'loop_: while (((*ptr.borrow()).read()) != 4) {
        let __rhs = ((*ptr.borrow()).read());
        (*out.borrow_mut()) += __rhs;
        (*ptr.borrow_mut()).postfix_inc();
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<i32>).offset(4 as isize)),
    ));
    'loop_: while (((*ptr.borrow()).read()) != 1) {
        let __rhs = ((*ptr.borrow()).read());
        (*out.borrow_mut()) += __rhs;
        (*ptr.borrow_mut()).postfix_dec();
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<i32>).offset(3 as isize)),
    ));
    'loop_: while (((*ptr.borrow()).read()) != 2) {
        let __rhs = ((*ptr.borrow()).read());
        (*out.borrow_mut()) += __rhs;
        (*ptr.borrow_mut()).prefix_dec();
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<i32>).offset(0 as isize)),
    ));
    'loop_: while (((*ptr.borrow()).read()) != 0) {
        let __rhs = ((*ptr.borrow()).read());
        (*out.borrow_mut()) += __rhs;
        let __rhs = (*ptr.borrow()).offset((1) as isize);
        (*ptr.borrow_mut()) = __rhs;
    }
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<i32>).offset(0 as isize)),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 5) {
        let __rhs = ((*ptr.borrow()).offset((*i.borrow()) as isize).read());
        (*out.borrow_mut()) += __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*out.borrow());
}
