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
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
    'loop_: while ((*x.borrow()) < 3) {
        (*a.borrow_mut())[((*x.borrow_mut()).postfix_inc()) as usize].prefix_inc();
    }
    let out: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*x.borrow()) != 0) {
        (*out.borrow_mut()) += (*a.borrow())[((*x.borrow_mut()).prefix_dec()) as usize];
    }
    (*out.borrow_mut()).postfix_inc();
    let x2: Value<i32> = Rc::new(RefCell::new((*out.borrow_mut()).prefix_dec()));
    (*out.borrow_mut()).prefix_inc();
    let x3: Value<i32> = Rc::new(RefCell::new((*out.borrow_mut()).postfix_dec()));
    return (((*out.borrow_mut()).postfix_inc() + (*x2.borrow())) + (*x3.borrow()));
}
