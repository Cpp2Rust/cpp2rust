extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_0(fn_: impl Fn(i32) -> i32, x: i32) -> i32 {
    let fn_: Value<_> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({
        let _x: i32 = (*x.borrow());
        (*fn_.borrow_mut())(_x)
    })
    .clone();
}
pub fn apply_1(fn_: impl Fn(i32) -> i32, x: i32) -> i32 {
    let fn_: Value<_> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({
        let _x: i32 = (*x.borrow());
        (*fn_.borrow_mut())(_x)
    })
    .clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let add_base: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) + (*base.borrow()));
        }),
    ));
    assert!(
        (({
            let _fn: _ = (*add_base.borrow()).clone();
            apply_0(_fn, 5)
        }) == 15)
    );
    (*base.borrow_mut()) = 100;
    assert!(
        (({
            let _fn: _ = (*add_base.borrow()).clone();
            apply_0(_fn, 5)
        }) == 105)
    );
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let scale: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) * (*factor.borrow()));
        }),
    ));
    assert!(
        (({
            let _fn: _ = (*scale.borrow()).clone();
            apply_1(_fn, 4)
        }) == 12)
    );
    return 0;
}
