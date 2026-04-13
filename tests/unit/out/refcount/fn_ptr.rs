extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_foo_0(p: AnyPtr) -> i32 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).cast::<i32>().expect("ub:wrong type").read());
}
pub fn foo_1(fn_: Option<fn(AnyPtr) -> i32>, pi: Ptr<i32>) -> i32 {
    let fn_: Value<Option<fn(AnyPtr) -> i32>> = Rc::new(RefCell::new(fn_));
    let pi: Value<Ptr<i32>> = Rc::new(RefCell::new(pi));
    return ({
        let _arg0: AnyPtr = ((*pi.borrow()).clone() as Ptr<i32>).to_any();
        (*fn_.borrow()).unwrap()(_arg0)
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fn_: Value<Option<fn(AnyPtr) -> i32>> = Rc::new(RefCell::new(None));
    assert!((*fn_.borrow()).is_none());
    assert!({
        let _lhs = (*fn_.borrow()).clone();
        _lhs != Some(my_foo_0 as _)
    });
    (*fn_.borrow_mut()) = Some(my_foo_0 as _);
    assert!(!((*fn_.borrow()).is_none()));
    assert!({
        let _lhs = (*fn_.borrow()).clone();
        _lhs == Some(my_foo_0 as _)
    });
    let a: Value<i32> = Rc::new(RefCell::new(10));
    assert!({
        let _lhs = ({
            let _fn: Option<fn(AnyPtr) -> i32> = (*fn_.borrow()).clone();
            let _pi: Ptr<i32> = (a.as_pointer());
            foo_1(_fn, _pi)
        });
        _lhs == (*a.borrow())
    });
    return 0;
}
