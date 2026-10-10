extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut out: i32 = 0;
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    (*p1.borrow()).write(1);
    out *= (*x.borrow());
    let p2: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new((p1.as_pointer())));
    ((*p2.borrow()).read()).write(2);
    out *= (*x.borrow());
    let mut p3: Ptr<Ptr<Ptr<i32>>> = (p2.as_pointer());
    ((p3.read()).read()).write(3);
    out *= (*x.borrow());
    return out;
}
pub fn __cpp2rust_init_globals() {}
