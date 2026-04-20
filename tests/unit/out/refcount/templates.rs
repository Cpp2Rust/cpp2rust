extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn foo_1(x: f64) -> f64 {
    let x: Value<f64> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn bar_2(p: Ptr<i32>, flag: bool) -> Ptr<i32> {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    let flag: Value<bool> = Rc::new(RefCell::new(flag));
    return if (*flag.borrow()) {
        (*p.borrow()).clone()
    } else {
        Default::default()
    };
}
pub fn bar_3(p: Ptr<f64>, flag: bool) -> Ptr<f64> {
    let p: Value<Ptr<f64>> = Rc::new(RefCell::new(p));
    let flag: Value<bool> = Rc::new(RefCell::new(flag));
    return if (*flag.borrow()) {
        (*p.borrow()).clone()
    } else {
        Default::default()
    };
}
pub fn func_4(x1: i32, x2: i32, x3: i32) -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(x1));
    let x2: Value<i32> = Rc::new(RefCell::new(x2));
    let x3: Value<i32> = Rc::new(RefCell::new(x3));
    return (((*x1.borrow()) + (*x2.borrow())) + (*x3.borrow()));
}
pub fn func_5(x1: f64, x2: i32, x3: f64) -> i32 {
    let x1: Value<f64> = Rc::new(RefCell::new(x1));
    let x2: Value<i32> = Rc::new(RefCell::new(x2));
    let x3: Value<f64> = Rc::new(RefCell::new(x3));
    return ((((*x1.borrow()) + ((*x2.borrow()) as f64)) + (*x3.borrow())) as i32);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(10));
    let y: Value<f64> = Rc::new(RefCell::new(((*x.borrow()) as f64)));
    return (((((((({
        let _x: i32 = (*x.borrow());
        foo_0(_x)
    }) as f64)
        + ({
            let _x: f64 = (*y.borrow());
            foo_1(_x)
        }))
        + ((({
            let _p: Ptr<i32> = (x.as_pointer());
            let _flag: bool = true;
            bar_2(_p, _flag)
        })
        .read()) as f64))
        + (({
            let _p: Ptr<f64> = (y.as_pointer());
            let _flag: bool = true;
            bar_3(_p, _flag)
        })
        .read()))
        + (({
            let _x1: i32 = 1;
            let _x2: i32 = 2;
            let _x3: i32 = 3;
            func_4(_x1, _x2, _x3)
        }) as f64))
        + (({
            let _x1: f64 = 2.0E+0;
            let _x2: i32 = (*x.borrow());
            let _x3: f64 = (*y.borrow());
            func_5(_x1, _x2, _x3)
        }) as f64)) as i32);
}
