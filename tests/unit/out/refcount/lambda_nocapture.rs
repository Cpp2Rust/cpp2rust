extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn call_0(f: Option<fn(i32, i32) -> i32>, a: i32, b: i32) -> i32 {
    let f: Value<Option<fn(i32, i32) -> i32>> = Rc::new(RefCell::new(f));
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ({
        let _arg0: i32 = (*a.borrow());
        let _arg1: i32 = (*b.borrow());
        (*f.borrow()).unwrap()(_arg0, _arg1)
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let add: Value<Option<fn(i32, i32) -> i32>> = Rc::new(RefCell::new(Some(|a: i32, b: i32| {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        return ((*a.borrow()) + (*b.borrow()));
    })));
    let sub: Value<Option<fn(i32, i32) -> i32>> = Rc::new(RefCell::new(Some(|a: i32, b: i32| {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        return ((*a.borrow()) - (*b.borrow()));
    })));
    assert!(!((*add.borrow()).is_none()));
    assert!({
        let _lhs = (*add.borrow()).clone();
        _lhs != (*sub.borrow()).clone()
    });
    assert!(
        (({
            let _arg0: i32 = 2;
            let _arg1: i32 = 3;
            (*add.borrow()).unwrap()(_arg0, _arg1)
        }) == 5)
    );
    assert!(
        (({
            let _arg0: i32 = 10;
            let _arg1: i32 = 4;
            (*sub.borrow()).unwrap()(_arg0, _arg1)
        }) == 6)
    );
    assert!(
        (({
            let _f: Option<fn(i32, i32) -> i32> = (*add.borrow()).clone();
            let _a: i32 = 7;
            let _b: i32 = 8;
            call_0(_f, _a, _b)
        }) == 15)
    );
    assert!(
        (({
            let _f: Option<fn(i32, i32) -> i32> = Some(|a: i32, b: i32| {
                let a: Value<i32> = Rc::new(RefCell::new(a));
                let b: Value<i32> = Rc::new(RefCell::new(b));
                return ((*a.borrow()) * (*b.borrow()));
            });
            let _a: i32 = 6;
            let _b: i32 = 7;
            call_0(_f, _a, _b)
        }) == 42)
    );
    return 0;
}
