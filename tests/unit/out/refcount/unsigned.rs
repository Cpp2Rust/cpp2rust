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
    let x: Value<u32> = Rc::new(RefCell::new((-1_i32 as u32)));
    assert!(((*x.borrow()) == <u32>::MAX));
    let v1: Value<i32> = Rc::new(RefCell::new((((*x.borrow()) & 1_u32) as i32)));
    let v2: Value<u32> = Rc::new(RefCell::new(((*x.borrow()) & 1_u32)));
    let p: Value<Ptr<u32>> = Rc::new(RefCell::new((x.as_pointer())));
    let b: Value<bool> = Rc::new(RefCell::new(((((*p.borrow()).read()) & 255_u32) != 0)));
    let a: Value<i32> = Rc::new(RefCell::new(((((*p.borrow()).read()) & 255_u32) as i32)));
    return (*a.borrow());
}
