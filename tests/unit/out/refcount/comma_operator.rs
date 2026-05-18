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
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new({
        (*x.borrow_mut()) = 2;
        ((*x.borrow()) + 1)
    }));
    assert!(((*x.borrow()) == 2));
    assert!(((*y.borrow()) == 3));
    let z: Value<i32> = Rc::new(RefCell::new({
        1;
        2;
        3
    }));
    assert!(((*z.borrow()) == 3));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let w: Value<i32> = Rc::new(RefCell::new({
        (*counter.borrow_mut()).postfix_inc();
        (*counter.borrow_mut()).postfix_inc();
        (*counter.borrow())
    }));
    assert!(((*counter.borrow()) == 2));
    assert!(((*w.borrow()) == 2));
    let a: Value<i32> = Rc::new(RefCell::new(0));
    let b: Value<i32> = Rc::new(RefCell::new(0));
    if {
        (*a.borrow_mut()) = 1;
        (*b.borrow_mut()) = 2;
        (((*a.borrow()) + (*b.borrow())) > 0)
    } {
        assert!(((*a.borrow()) == 1));
        assert!(((*b.borrow()) == 2));
    }
    return 0;
}
