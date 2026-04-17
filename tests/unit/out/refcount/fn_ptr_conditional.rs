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
pub fn pick_3(mode: i32) -> FnPtr<fn(i32) -> i32> {
    let mode: Value<i32> = Rc::new(RefCell::new(mode));
    return if ((*mode.borrow()) > 0) {
        fn_ptr!(inc_0, fn(i32) -> i32)
    } else {
        if ((*mode.borrow()) < 0) {
            fn_ptr!(dec_1, fn(i32) -> i32)
        } else {
            fn_ptr!(identity_2, fn(i32) -> i32)
        }
    };
}
pub fn apply_4(fn_: FnPtr<fn(i32) -> i32>, x: i32) -> i32 {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let actual: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(if !(*fn_.borrow()).is_null() {
            (*fn_.borrow()).clone()
        } else {
            fn_ptr!(identity_2, fn(i32) -> i32)
        }));
    return ({
        let _arg0: i32 = (*x.borrow());
        (*(*actual.borrow()))(_arg0)
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _arg0: i32 = 10;
            (*({
                let _mode: i32 = 1;
                pick_3(_mode)
            }))(_arg0)
        }) == 11)
    );
    assert!(
        (({
            let _arg0: i32 = 10;
            (*({
                let _mode: i32 = -1_i32;
                pick_3(_mode)
            }))(_arg0)
        }) == 9)
    );
    assert!(
        (({
            let _arg0: i32 = 10;
            (*({
                let _mode: i32 = 0;
                pick_3(_mode)
            }))(_arg0)
        }) == 10)
    );
    assert!(
        (({
            let _fn: FnPtr<fn(i32) -> i32> = fn_ptr!(inc_0, fn(i32) -> i32);
            let _x: i32 = 5;
            apply_4(_fn, _x)
        }) == 6)
    );
    assert!(
        (({
            let _fn: FnPtr<fn(i32) -> i32> = FnPtr::null();
            let _x: i32 = 5;
            apply_4(_fn, _x)
        }) == 5)
    );
    return 0;
}
