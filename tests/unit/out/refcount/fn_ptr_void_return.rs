extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
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
    ({ (*(*fn_.borrow()))((*x.borrow()).clone()) });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(42));
    ({ run_2(FnPtr::<fn(Ptr<i32>)>::new(negate_0), (a.as_pointer())) });
    assert!(((*a.borrow()) == -42_i32));
    ({ run_2(FnPtr::<fn(Ptr<i32>)>::new(zero_out_1), (a.as_pointer())) });
    assert!(((*a.borrow()) == 0));
    let fn_: Value<FnPtr<fn(Ptr<i32>)>> =
        Rc::new(RefCell::new(FnPtr::<fn(Ptr<i32>)>::new(negate_0)));
    assert!(!((*fn_.borrow()).is_null()));
    let b: Value<i32> = Rc::new(RefCell::new(10));
    ({ (*(*fn_.borrow()))((b.as_pointer())) });
    assert!(((*b.borrow()) == -10_i32));
    return 0;
}
