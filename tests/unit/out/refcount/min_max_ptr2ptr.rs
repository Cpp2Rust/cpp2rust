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
    let a: Value<i32> = Rc::new(RefCell::new(10));
    let b: Value<i32> = Rc::new(RefCell::new(20));
    let pa: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    let pb: Value<Ptr<i32>> = Rc::new(RefCell::new((b.as_pointer())));
    let ppa: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new((pa.as_pointer())));
    let ppb: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new((pb.as_pointer())));
    let r1: Value<i32> = Rc::new(RefCell::new(
        (if ((*ppa.borrow()).read()).clone().read() >= ((*ppb.borrow()).read()).clone().read() {
            ((*ppa.borrow()).read()).clone()
        } else {
            ((*ppb.borrow()).read()).clone()
        }
        .read()),
    ));
    let r2: Value<i32> = Rc::new(RefCell::new(
        (if ((*ppa.borrow()).read()).clone().read() <= ((*ppb.borrow()).read()).clone().read() {
            ((*ppa.borrow()).read()).clone()
        } else {
            ((*ppb.borrow()).read()).clone()
        }
        .read()),
    ));
    return ((*r1.borrow()) + (*r2.borrow()));
}
