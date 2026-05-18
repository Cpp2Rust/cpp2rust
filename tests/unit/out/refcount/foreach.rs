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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 10) {
        {
            let a0_clone = (*i.borrow()).clone();
            (*v.borrow_mut()).push(a0_clone)
        };
        (*i.borrow_mut()).prefix_inc();
    }
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for mut x in v.as_pointer() as Ptr<i32> {
        let x: Value<i32> = Rc::new(RefCell::new(x.read().clone()));
        (*sum.borrow_mut()) += (*x.borrow());
    }
    return (*sum.borrow());
}
