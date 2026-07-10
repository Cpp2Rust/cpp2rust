extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(x: Ptr<i32>) {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    {
        let _ptr = (*x.borrow()).clone();
        _ptr.write(_ptr.read() * 2)
    };
}
pub fn maybe_call_1(cb: FnPtr<fn(Ptr<i32>)>, x: Ptr<i32>) {
    let cb: Value<FnPtr<fn(Ptr<i32>)>> = Rc::new(RefCell::new(cb));
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    if !(*cb.borrow()).is_null() {
        ({ (*(*cb.borrow()))((*x.borrow()).clone()) });
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(5));
    ({ maybe_call_1(FnPtr::<fn(Ptr<i32>)>::new(double_it_0), (a.as_pointer())) });
    assert!(((*a.borrow()) == 10));
    let b: Value<i32> = Rc::new(RefCell::new(5));
    ({ maybe_call_1(FnPtr::null(), (b.as_pointer())) });
    assert!(((*b.borrow()) == 5));
    let fn_: Value<FnPtr<fn(Ptr<i32>)>> = Rc::new(RefCell::new(FnPtr::null()));
    if !!(*fn_.borrow()).is_null() {
        (*fn_.borrow_mut()) = FnPtr::<fn(Ptr<i32>)>::new(double_it_0);
    }
    let c: Value<i32> = Rc::new(RefCell::new(3));
    if !(*fn_.borrow()).is_null() {
        ({ (*(*fn_.borrow()))((c.as_pointer())) });
    }
    assert!(((*c.borrow()) == 6));
    return 0;
}
