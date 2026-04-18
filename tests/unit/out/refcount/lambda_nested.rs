extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
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
            return ({
                let _z: i32 = 1;
                (*inner.borrow_mut())(_z)
            })
            .clone();
        }),
    ));
    assert!(
        (({
            let _y: i32 = 20;
            (*outer.borrow_mut())(_y)
        }) == 31)
    );
    (*x.borrow_mut()) = 100;
    assert!(
        (({
            let _y: i32 = 20;
            (*outer.borrow_mut())(_y)
        }) == 121)
    );
    return 0;
}
