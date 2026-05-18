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
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    if {
        let _lhs = (*p1.borrow()).clone();
        _lhs != (*p2.borrow()).clone()
    } {
        return 1;
    } else {
        return -1_i32;
    }
    panic!("ub: non-void function does not return a value")
}
