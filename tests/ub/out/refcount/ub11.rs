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
    let element: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(10)));
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new((*element.borrow()).offset((1) as isize)));
    let out: Value<i32> = Rc::new(RefCell::new(((*ptr.borrow()).read())));
    (*element.borrow()).delete();
    return (*out.borrow());
}
