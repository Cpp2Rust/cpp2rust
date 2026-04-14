extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn negate_0(x: Ptr<i32>) {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    let __rhs = -((*x.borrow()).read());
    (*x.borrow()).write(__rhs);
}
pub fn zero_out_1(x: Ptr<i32>) {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    (*x.borrow()).write(0);
}
pub fn run_2(fn_: FnPtr<fn(Ptr<i32>)>, x: Ptr<i32>) {
    let fn_: Value<FnPtr<fn(Ptr<i32>)>> = Rc::new(RefCell::new(fn_));
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    ({
        let _arg0: Ptr<i32> = (*x.borrow()).clone();
        (*fn_.borrow()).call()(_arg0)
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(42));
    ({
        let _fn: FnPtr<fn(Ptr<i32>)> = fn_ptr!(negate_0, fn(Ptr::<i32>));
        let _x: Ptr<i32> = (a.as_pointer());
        run_2(_fn, _x)
    });
    assert!(((*a.borrow()) == -42_i32));
    ({
        let _fn: FnPtr<fn(Ptr<i32>)> = fn_ptr!(zero_out_1, fn(Ptr::<i32>));
        let _x: Ptr<i32> = (a.as_pointer());
        run_2(_fn, _x)
    });
    assert!(((*a.borrow()) == 0));
    let fn_: Value<FnPtr<fn(Ptr<i32>)>> = Rc::new(RefCell::new(fn_ptr!(negate_0, fn(Ptr::<i32>))));
    assert!(!((*fn_.borrow()).is_null()));
    let b: Value<i32> = Rc::new(RefCell::new(10));
    ({
        let _arg0: Ptr<i32> = (b.as_pointer());
        (*fn_.borrow()).call()(_arg0)
    });
    assert!(((*b.borrow()) == -10_i32));
    return 0;
}
