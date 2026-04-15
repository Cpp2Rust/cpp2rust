extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(x: Ptr<i32>) {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    {
        let __ptr = (*x.borrow()).clone();
        let __tmp = __ptr.read() * 2;
        __ptr.write(__tmp)
    };
}
pub fn maybe_call_1(cb: FnPtr<fn(Ptr<i32>)>, x: Ptr<i32>) {
    let cb: Value<FnPtr<fn(Ptr<i32>)>> = Rc::new(RefCell::new(cb));
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    if !(*cb.borrow()).is_null() {
        ({
            let _arg0: Ptr<i32> = (*x.borrow()).clone();
            (*(*cb.borrow()))(_arg0)
        });
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(5));
    ({
        let _cb: FnPtr<fn(Ptr<i32>)> = fn_ptr!(double_it_0, fn(Ptr::<i32>));
        let _x: Ptr<i32> = (a.as_pointer());
        maybe_call_1(_cb, _x)
    });
    assert!(((*a.borrow()) == 10));
    let b: Value<i32> = Rc::new(RefCell::new(5));
    ({
        let _cb: FnPtr<fn(Ptr<i32>)> = FnPtr::null();
        let _x: Ptr<i32> = (b.as_pointer());
        maybe_call_1(_cb, _x)
    });
    assert!(((*b.borrow()) == 5));
    let fn_: Value<FnPtr<fn(Ptr<i32>)>> = Rc::new(RefCell::new(FnPtr::null()));
    if !!(*fn_.borrow()).is_null() {
        (*fn_.borrow_mut()) = (fn_ptr!(double_it_0, fn(Ptr::<i32>))).clone();
    }
    let c: Value<i32> = Rc::new(RefCell::new(3));
    if !(*fn_.borrow()).is_null() {
        ({
            let _arg0: Ptr<i32> = (c.as_pointer());
            (*(*fn_.borrow()))(_arg0)
        });
    }
    assert!(((*c.borrow()) == 6));
    return 0;
}
