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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 10) {
        {
            let a0_clone = (*i.borrow()).clone();
            (*v.borrow_mut()).push(a0_clone)
        };
        (*i.borrow_mut()).prefix_inc();
    }
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: for mut x in v.as_pointer() as Ptr<i32> {
        let x: Value<i32> = Rc::new(RefCell::new(x.read()));
        (*sum.borrow_mut()) += (*x.borrow());
    }
    assert!(((*sum.borrow()) == 45));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
