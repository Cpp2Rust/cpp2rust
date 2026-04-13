extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn triple_it_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 3);
}
thread_local!(
    pub static g_op: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(None));
);
pub fn set_op_2(fn_: Option<fn(i32) -> i32>) {
    let fn_: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    (*g_op.with(Value::clone).borrow_mut()) = (*fn_.borrow()).clone();
}
pub fn call_op_3(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    if !(*g_op.with(Value::clone).borrow()).is_none() {
        return ({
            let _arg0: i32 = (*x.borrow());
            (*g_op.with(Value::clone).borrow()).unwrap()(_arg0)
        });
    }
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _x: i32 = 5;
            call_op_3(_x)
        }) == 5)
    );
    ({
        let _fn: Option<fn(i32) -> i32> = Some(double_it_0 as _);
        set_op_2(_fn)
    });
    assert!(!((*g_op.with(Value::clone).borrow()).is_none()));
    assert!({
        let _lhs = (*g_op.with(Value::clone).borrow()).clone();
        _lhs == Some(double_it_0 as _)
    });
    assert!(
        (({
            let _x: i32 = 5;
            call_op_3(_x)
        }) == 10)
    );
    ({
        let _fn: Option<fn(i32) -> i32> = Some(triple_it_1 as _);
        set_op_2(_fn)
    });
    assert!({
        let _lhs = (*g_op.with(Value::clone).borrow()).clone();
        _lhs == Some(triple_it_1 as _)
    });
    assert!(
        (({
            let _x: i32 = 5;
            call_op_3(_x)
        }) == 15)
    );
    ({
        let _fn: Option<fn(i32) -> i32> = None;
        set_op_2(_fn)
    });
    assert!((*g_op.with(Value::clone).borrow()).is_none());
    assert!(
        (({
            let _x: i32 = 5;
            call_op_3(_x)
        }) == 5)
    );
    return 0;
}
