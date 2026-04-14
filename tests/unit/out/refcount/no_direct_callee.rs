extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0() -> bool {
    return false;
}
pub fn test_1(fn_: FnPtr<fn() -> bool>) -> i32 {
    let fn_: Value<FnPtr<fn() -> bool>> = Rc::new(RefCell::new(fn_));
    if !({ (*fn_.borrow()).call()() }) {
        return 1;
    }
    return 0;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return ({
        let _fn: FnPtr<fn() -> bool> = fn_ptr!(test1_0, fn() -> bool);
        test_1(_fn)
    });
}
