extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new(2));
    let x3: Value<i32> = Rc::new(RefCell::new(10));
    let x4: Value<i32> = Rc::new(RefCell::new(20));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x1.as_pointer())));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((x2.as_pointer())));
    let r1: Value<i32> = Rc::new(RefCell::new(
        (if x1.as_pointer().read() >= x2.as_pointer().read() {
            x1.as_pointer()
        } else {
            x2.as_pointer()
        }
        .read()),
    ));
    let r2: Value<i32> = Rc::new(RefCell::new(
        (if x3.as_pointer().read() <= x4.as_pointer().read() {
            x3.as_pointer()
        } else {
            x4.as_pointer()
        }
        .read()),
    ));
    let r3: Value<i32> = Rc::new(RefCell::new(
        (if (*p1.borrow()).clone().read() >= x2.as_pointer().read() {
            (*p1.borrow()).clone()
        } else {
            x2.as_pointer()
        }
        .read()),
    ));
    let r4: Value<i32> = Rc::new(RefCell::new(
        (if (*p2.borrow()).clone().read() <= x3.as_pointer().read() {
            (*p2.borrow()).clone()
        } else {
            x3.as_pointer()
        }
        .read()),
    ));
    let r5: Value<i32> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(30));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(40));
        (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    return (((((*r1.borrow()) + (*r2.borrow())) + (*r3.borrow())) + (*r4.borrow()))
        + (*r5.borrow()));
}
