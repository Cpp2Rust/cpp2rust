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
    let N: Value<i32> = Rc::new(RefCell::new(5));
    let arr1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0, 0])));
    let arr2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 1, 1, 1, 1])));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = ((*i.borrow()) + (*arr2.borrow())[(*i.borrow()) as usize]);
        (*arr1.borrow_mut())[(*i.borrow()) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    let fatorial: Value<i32> = Rc::new(RefCell::new(1));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        (*fatorial.borrow_mut()) *= (*arr1.borrow())[(*i.borrow()) as usize];
        (*i.borrow_mut()).prefix_inc();
    }
    return (*fatorial.borrow());
}
