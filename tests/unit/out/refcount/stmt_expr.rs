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
    let x: Value<i32> = Rc::new(RefCell::new({
        let a: Value<i32> = Rc::new(RefCell::new(1));
        let b: Value<i32> = Rc::new(RefCell::new(2));
        ((*a.borrow()) + (*b.borrow()))
    }));
    assert!(((*x.borrow()) == 3));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let y: Value<i32> = Rc::new(RefCell::new({
        (*counter.borrow_mut()).postfix_inc();
        ((*counter.borrow()) * 10)
    }));
    assert!(((*y.borrow()) == 10));
    assert!(((*counter.borrow()) == 1));
    let z: Value<i32> = Rc::new(RefCell::new({
        let v: Value<i32> = Rc::new(RefCell::new(5));
        if ((*v.borrow()) > 0) {
            (*v.borrow_mut()) = { ((*v.borrow()) * 2) };
        }
        (*v.borrow())
    }));
    assert!(((*z.borrow()) == 10));
    assert!(
        ({
            let inner: Value<i32> = Rc::new(RefCell::new({
                let a: Value<i32> = Rc::new(RefCell::new(100));
                (*a.borrow())
            }));
            (*inner.borrow())
        } == 100)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
