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
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i16> = Rc::new(RefCell::new(2_i16));
    let x3: Value<u32> = Rc::new(RefCell::new(4_u32));
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(1);
    (*v.borrow_mut()).push(2);
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for mut elem in v.as_pointer() as Ptr<i32> {
        let elem: Value<i32> = Rc::new(RefCell::new(elem.read().clone()));
        let rhs_0 = (((*sum.borrow()) as i32) + (*elem.borrow())) as i32;
        (*sum.borrow_mut()) = rhs_0;
    }
    return (*sum.borrow());
}
