extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_foo_0(p: AnyPtr) -> i32 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).reinterpret_cast::<i32>().read());
}
pub fn foo_1(fn_: FnPtr<fn(AnyPtr) -> i32>, pi: Ptr<i32>) -> i32 {
    let fn_: Value<FnPtr<fn(AnyPtr) -> i32>> = Rc::new(RefCell::new(fn_));
    let pi: Value<Ptr<i32>> = Rc::new(RefCell::new(pi));
    return ({ (*(*fn_.borrow()))(((*pi.borrow()).clone() as Ptr<i32>).to_any()) });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fn_: Value<FnPtr<fn(AnyPtr) -> i32>> = Rc::new(RefCell::new(FnPtr::null()));
    assert!((*fn_.borrow()).is_null());
    assert!({
        let _lhs = (*fn_.borrow()).clone();
        _lhs != FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0)
    });
    (*fn_.borrow_mut()) = FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0);
    assert!(!((*fn_.borrow()).is_null()));
    assert!({
        let _lhs = (*fn_.borrow()).clone();
        _lhs == FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0)
    });
    let a: Value<i32> = Rc::new(RefCell::new(10));
    assert!({
        let _lhs = ({ foo_1((*fn_.borrow()).clone(), (a.as_pointer())) });
        _lhs == (*a.borrow())
    });
    return 0;
}
