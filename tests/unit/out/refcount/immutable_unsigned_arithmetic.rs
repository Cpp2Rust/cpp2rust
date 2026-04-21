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
    let a: Value<u32> = Rc::new(RefCell::new(0_u32));
    let p: Value<Ptr<u32>> = Rc::new(RefCell::new((a.as_pointer())));
    let rhs_0 = ((*p.borrow()).read()).wrapping_add(((*p.borrow()).read()));
    (*p.borrow()).write(rhs_0);
    return (((*p.borrow()).read()) as i32);
}
