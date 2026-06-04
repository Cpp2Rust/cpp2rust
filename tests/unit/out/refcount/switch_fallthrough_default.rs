extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_default_0(x: i32, flag: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let flag: Value<i32> = Rc::new(RefCell::new(flag));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        __v if __v == 7 => {
            if ((*flag.borrow()) != 0) {
                (*r.borrow_mut()) = 100;
                break;
            };
        }
        _ => {
            (*r.borrow_mut()) = 42;
            break;
        }
    });
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ fallthrough_default_0(7, 0,) }) == 42));
    assert!((({ fallthrough_default_0(7, 1,) }) == 100));
    assert!((({ fallthrough_default_0(99, 0,) }) == 42));
    return 0;
}
