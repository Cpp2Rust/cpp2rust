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
    let x: Value<i32> = Rc::new(RefCell::new(10));
    let outer: Value<_> = Rc::new(RefCell::new(
        (|y: i32| {
            let y: Value<i32> = Rc::new(RefCell::new(y));
            let inner: Value<_> = Rc::new(RefCell::new(
                (|z: i32| {
                    let z: Value<i32> = Rc::new(RefCell::new(z));
                    return (((*x.borrow()) + (*y.borrow())) + (*z.borrow()));
                }),
            ));
            return ({ (*inner.borrow_mut())(1) });
        }),
    ));
    assert!((({ (*outer.borrow_mut())(20,) }) == 31));
    (*x.borrow_mut()) = 100;
    assert!((({ (*outer.borrow_mut())(20,) }) == 121));
    return 0;
}
