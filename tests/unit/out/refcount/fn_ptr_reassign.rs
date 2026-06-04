extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_0(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) + (*b.borrow()));
}
pub fn sub_1(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) - (*b.borrow()));
}
pub fn mul_2(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) * (*b.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fn_: Value<FnPtr<fn(i32, i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32, i32) -> i32>::new(add_0)));
    assert!((({ (*(*fn_.borrow()))(3, 4,) }) == 7));
    (*fn_.borrow_mut()) = FnPtr::<fn(i32, i32) -> i32>::new(sub_1);
    assert!((({ (*(*fn_.borrow()))(10, 3,) }) == 7));
    (*fn_.borrow_mut()) = FnPtr::<fn(i32, i32) -> i32>::new(mul_2);
    assert!((({ (*(*fn_.borrow()))(6, 7,) }) == 42));
    (*fn_.borrow_mut()) = FnPtr::null();
    assert!((*fn_.borrow()).is_null());
    (*fn_.borrow_mut()) = FnPtr::<fn(i32, i32) -> i32>::new(add_0);
    assert!(!((*fn_.borrow()).is_null()));
    assert!((({ (*(*fn_.borrow()))(1, 1,) }) == 2));
    return 0;
}
