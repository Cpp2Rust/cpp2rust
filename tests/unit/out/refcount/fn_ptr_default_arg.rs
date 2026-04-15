extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
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
        return ({
            let _arg0: i32 = (*x.borrow());
            (*(*fn_.borrow()))(_arg0)
        });
    }
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _x: i32 = 5;
            let _fn: FnPtr<fn(i32) -> i32> = Default::default();
            apply_1(_x, Some(_fn))
        }) == 5)
    );
    assert!(
        (({
            let _x: i32 = 5;
            let _fn: FnPtr<fn(i32) -> i32> = FnPtr::null();
            apply_1(_x, Some(_fn))
        }) == 5)
    );
    assert!(
        (({
            let _x: i32 = 5;
            let _fn: FnPtr<fn(i32) -> i32> = fn_ptr!(identity_0, fn(i32) -> i32);
            apply_1(_x, Some(_fn))
        }) == 5)
    );
    let negate: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_ptr_anon!(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return -(*x.borrow());
        }),
        fn(i32) -> i32
    )));
    assert!(
        (({
            let _x: i32 = 5;
            let _fn: FnPtr<fn(i32) -> i32> = (*negate.borrow()).clone();
            apply_1(_x, Some(_fn))
        }) == -5_i32)
    );
    return 0;
}
