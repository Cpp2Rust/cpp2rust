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
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let b_ptr: Value<Ptr<i32>> = Rc::new(RefCell::new((b.as_pointer())));
    (*b_ptr.borrow()).write(3);
    let b_ptr_ptr: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new((b_ptr.as_pointer())));
    ((*b_ptr_ptr.borrow()).read()).write(4);
    let __rhs = (((*b_ptr_ptr.borrow()).read()).read());
    (*b_ptr.borrow()).write(__rhs);
    let offset: Value<u64> = Rc::new(RefCell::new(
        ((((*b_ptr.borrow()).clone() - (*b_ptr.borrow()).clone()) as i64) as u64),
    ));
    return (*b.borrow());
}
