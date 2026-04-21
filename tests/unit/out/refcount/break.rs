extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_test_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    let j: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        (*x.borrow_mut()) += 1;
        if ((*x.borrow()) == 100) {
            break;
        }
        let k: Value<i32> = Rc::new(RefCell::new(0));
        let w: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*w.borrow()) < (*j.borrow())) {
            break;
            (*w.borrow_mut()) += 1;
            (*k.borrow_mut()) += 1;
            (*i.borrow_mut()) += (*k.borrow());
        }
        let __rhs = ((*x.borrow()) + 1);
        (*x.borrow_mut()) = __rhs;
        (*j.borrow_mut()) = (*i.borrow());
        (*i.borrow_mut()) += 1;
    }
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return ({
        let _n: i32 = 200;
        for_test_0(_n)
    });
}
