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
    let N: Value<i64> = Rc::new(RefCell::new(25000000000_i64));
    let sum: Value<i64> = Rc::new(RefCell::new(0_i64));
    let i: Value<i64> = Rc::new(RefCell::new(0_i64));
    let j: Value<i64> = Rc::new(RefCell::new((*N.borrow())));
    'loop_: while ((*i.borrow()) < (*j.borrow())) {
        (*sum.borrow_mut()) += ((*i.borrow()) + (*j.borrow()));
        (*i.borrow_mut()).prefix_inc();
        (*j.borrow_mut()).prefix_dec();
    }
    return ((*sum.borrow()) as i32);
}
