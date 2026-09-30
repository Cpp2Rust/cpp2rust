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
    return ({ (*fn_.borrow()).call((*pi.borrow()).to_any()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fn_: Value<FnPtr<fn(AnyPtr) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(AnyPtr) -> i32>::null()));
    assert!((*fn_.borrow()).is_null());
    assert!(({ (*fn_.borrow()).clone() } != { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }));
    (*fn_.borrow_mut()) = FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0);
    assert!(!((*fn_.borrow()).is_null()));
    assert!(({ (*fn_.borrow()).clone() } == { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }));
    let a: Value<i32> = Rc::new(RefCell::new(10));
    assert!(({ ({ foo_1((*fn_.borrow()).clone(), (a.as_pointer()),) }) } == { (*a.borrow()) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
