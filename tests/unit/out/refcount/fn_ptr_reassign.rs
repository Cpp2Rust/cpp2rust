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
    assert!(
        (({
            let _arg0: i32 = 3;
            let _arg1: i32 = 4;
            (*(*fn_.borrow()))(_arg0, _arg1)
        }) == 7)
    );
    (*fn_.borrow_mut()) = FnPtr::<fn(i32, i32) -> i32>::new(sub_1);
    assert!(
        (({
            let _arg0: i32 = 10;
            let _arg1: i32 = 3;
            (*(*fn_.borrow()))(_arg0, _arg1)
        }) == 7)
    );
    (*fn_.borrow_mut()) = FnPtr::<fn(i32, i32) -> i32>::new(mul_2);
    assert!(
        (({
            let _arg0: i32 = 6;
            let _arg1: i32 = 7;
            (*(*fn_.borrow()))(_arg0, _arg1)
        }) == 42)
    );
    (*fn_.borrow_mut()) = FnPtr::null();
    assert!((*fn_.borrow()).is_null());
    (*fn_.borrow_mut()) = FnPtr::<fn(i32, i32) -> i32>::new(add_0);
    assert!(!((*fn_.borrow()).is_null()));
    assert!(
        (({
            let _arg0: i32 = 1;
            let _arg1: i32 = 1;
            (*(*fn_.borrow()))(_arg0, _arg1)
        }) == 2)
    );
    return 0;
}
