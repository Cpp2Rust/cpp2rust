extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
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
        FnPtr::<fn(i32) -> i32>::new(inc_0)
    } else {
        if ((*mode.borrow()) < 0) {
            FnPtr::<fn(i32) -> i32>::new(dec_1)
        } else {
            FnPtr::<fn(i32) -> i32>::new(identity_2)
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
            FnPtr::<fn(i32) -> i32>::new(identity_2)
        }));
    return ({ (*(*actual.borrow()))((*x.borrow())) });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ (*({ pick_3(1,) }))(10,) }) == 11));
    assert!((({ (*({ pick_3(-1_i32,) }))(10,) }) == 9));
    assert!((({ (*({ pick_3(0,) }))(10,) }) == 10));
    assert!((({ apply_4(FnPtr::<fn(i32) -> i32>::new(inc_0), 5,) }) == 6));
    assert!((({ apply_4(FnPtr::null(), 5,) }) == 5));
    return 0;
}
