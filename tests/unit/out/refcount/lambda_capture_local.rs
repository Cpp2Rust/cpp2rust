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
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let add_base: Value<Rc<dyn Fn(i32) -> i32>> = Rc::new(RefCell::new(Rc::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) + (*base.borrow()));
        }),
    )));
    let scale: Value<Rc<dyn Fn(i32) -> i32>> = Rc::new(RefCell::new(Rc::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) * (*factor.borrow()));
        }),
    )));
    assert!(
        (({
            let _x: i32 = 5;
            (*add_base.borrow())(_x)
        }) == 15)
    );
    assert!(
        (({
            let _x: i32 = 4;
            (*scale.borrow())(_x)
        }) == 12)
    );
    (*base.borrow_mut()) = 100;
    assert!(
        (({
            let _x: i32 = 5;
            (*add_base.borrow())(_x)
        }) == 105)
    );
    assert!(
        (({
            let _x: i32 = 4;
            (*scale.borrow())(_x)
        }) == 12)
    );
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let accumulate: Value<Rc<dyn Fn(i32)>> = Rc::new(RefCell::new(Rc::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            (*sum.borrow_mut()) += (*x.borrow());
        }),
    )));
    ({
        let _x: i32 = 1;
        (*accumulate.borrow())(_x)
    });
    ({
        let _x: i32 = 2;
        (*accumulate.borrow())(_x)
    });
    ({
        let _x: i32 = 3;
        (*accumulate.borrow())(_x)
    });
    assert!(((*sum.borrow()) == 6));
    return 0;
}
