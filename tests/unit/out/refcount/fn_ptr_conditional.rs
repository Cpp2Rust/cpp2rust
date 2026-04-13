extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) + 1);
}
pub fn dec_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) - 1);
}
pub fn identity_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn pick_3(mode: i32) -> Option<fn(i32) -> i32> {
    let mode: Value<i32> = Rc::new(RefCell::new(mode));
    return if ((*mode.borrow()) > 0) {
        Some(inc_0 as _)
    } else {
        if ((*mode.borrow()) < 0) {
            Some(dec_1 as _)
        } else {
            Some(identity_2 as _)
        }
    };
}
pub fn apply_4(fn_: Option<fn(i32) -> i32>, x: i32) -> i32 {
    let fn_: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let actual: Value<Option<fn(i32) -> i32>> =
        Rc::new(RefCell::new(if !(*fn_.borrow()).is_none() {
            (*fn_.borrow()).clone()
        } else {
            Some(identity_2 as _)
        }));
    return ({
        let _arg0: i32 = (*x.borrow());
        (*actual.borrow()).unwrap()(_arg0)
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _arg0: i32 = 10;
            ({
                let _mode: i32 = 1;
                pick_3(_mode)
            })
            .unwrap()(_arg0)
        }) == 11)
    );
    assert!(
        (({
            let _arg0: i32 = 10;
            ({
                let _mode: i32 = -1_i32;
                pick_3(_mode)
            })
            .unwrap()(_arg0)
        }) == 9)
    );
    assert!(
        (({
            let _arg0: i32 = 10;
            ({
                let _mode: i32 = 0;
                pick_3(_mode)
            })
            .unwrap()(_arg0)
        }) == 10)
    );
    assert!(
        (({
            let _fn: Option<fn(i32) -> i32> = Some(inc_0 as _);
            let _x: i32 = 5;
            apply_4(_fn, _x)
        }) == 6)
    );
    assert!(
        (({
            let _fn: Option<fn(i32) -> i32> = None;
            let _x: i32 = 5;
            apply_4(_fn, _x)
        }) == 5)
    );
    return 0;
}
