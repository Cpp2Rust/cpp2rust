extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn stacked_with_inner_fallthrough_0(x: i32, flag: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let flag: Value<i32> = Rc::new(RefCell::new(flag));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 1 || v == 2 || v == 3 => {
            if (!((*flag.borrow()) != 0) as bool) {
                (*r.borrow_mut()) = 50;
                break;
            };
        }
        _ => {
            (*r.borrow_mut()) = 999;
            break;
        }
    });
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _x: i32 = 1;
            let _flag: i32 = 0;
            stacked_with_inner_fallthrough_0(_x, _flag)
        }) == 50)
    );
    assert!(
        (({
            let _x: i32 = 2;
            let _flag: i32 = 1;
            stacked_with_inner_fallthrough_0(_x, _flag)
        }) == 999)
    );
    assert!(
        (({
            let _x: i32 = 99;
            let _flag: i32 = 0;
            stacked_with_inner_fallthrough_0(_x, _flag)
        }) == 999)
    );
    return 0;
}
