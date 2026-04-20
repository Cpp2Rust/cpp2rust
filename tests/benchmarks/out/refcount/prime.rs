extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_prime_0(x: i32) -> bool {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let i: Value<i32> = Rc::new(RefCell::new(2));
    'loop_: while ((*i.borrow()) < (*x.borrow())) {
        if (((*x.borrow()) % (*i.borrow())) == 0) {
            return false;
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return true;
}
pub fn largest_prime_1(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let max: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        if ({
            let _x: i32 = (*i.borrow());
            is_prime_0(_x)
        }) {
            (*max.borrow_mut()) = (*i.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*max.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<i32> = Rc::new(RefCell::new(270000));
    let largest: Value<i32> = Rc::new(RefCell::new(
        ({
            let _n: i32 = (*N.borrow());
            largest_prime_1(_n)
        }),
    ));
    return (*largest.borrow());
}
