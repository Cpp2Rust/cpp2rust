extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
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
    pub static g_op: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::null()));
);
pub fn set_op_2(fn_: FnPtr<fn(i32) -> i32>) {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    (*g_op.with(Value::clone).borrow_mut()) = (*fn_.borrow()).clone();
}
pub fn call_op_3(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    if !(*g_op.with(Value::clone).borrow()).is_null() {
        return ({
            let _arg0: i32 = (*x.borrow());
            (*(*g_op.with(Value::clone).borrow()))(_arg0)
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
        let _fn: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(double_it_0);
        set_op_2(_fn)
    });
    assert!(!((*g_op.with(Value::clone).borrow()).is_null()));
    assert!({
        let _lhs = (*g_op.with(Value::clone).borrow()).clone();
        _lhs == FnPtr::<fn(i32) -> i32>::new(double_it_0)
    });
    assert!(
        (({
            let _x: i32 = 5;
            call_op_3(_x)
        }) == 10)
    );
    ({
        let _fn: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(triple_it_1);
        set_op_2(_fn)
    });
    assert!({
        let _lhs = (*g_op.with(Value::clone).borrow()).clone();
        _lhs == FnPtr::<fn(i32) -> i32>::new(triple_it_1)
    });
    assert!(
        (({
            let _x: i32 = 5;
            call_op_3(_x)
        }) == 15)
    );
    ({
        let _fn: FnPtr<fn(i32) -> i32> = FnPtr::null();
        set_op_2(_fn)
    });
    assert!((*g_op.with(Value::clone).borrow()).is_null());
    assert!(
        (({
            let _x: i32 = 5;
            call_op_3(_x)
        }) == 5)
    );
    return 0;
}
