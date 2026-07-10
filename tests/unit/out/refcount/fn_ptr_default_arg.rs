extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn apply_1(x: i32, fn_: Option<FnPtr<fn(i32) -> i32>>) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_.unwrap_or(FnPtr::null())));
    if !(*fn_.borrow()).is_null() {
        return ({ (*(*fn_.borrow()))((*x.borrow())) });
    }
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ apply_1(5, Some(Default::default()),) }) == 5));
    assert!((({ apply_1(5, Some(FnPtr::null()),) }) == 5));
    assert!((({ apply_1(5, Some(FnPtr::<fn(i32) -> i32>::new(identity_0)),) }) == 5));
    let negate: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return -(*x.borrow());
        }),
    )));
    assert!((({ apply_1(5, Some((*negate.borrow()).clone()),) }) == -5_i32));
    return 0;
}
