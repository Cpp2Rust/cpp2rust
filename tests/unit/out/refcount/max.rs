extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new(2));
    let x3: Value<i32> = Rc::new(RefCell::new(10));
    let x4: Value<i32> = Rc::new(RefCell::new(20));
    let mut p1: Ptr<i32> = (x1.as_pointer());
    let mut p2: Ptr<i32> = (x2.as_pointer());
    let mut r1: i32 = (if x1.as_pointer().read() >= x2.as_pointer().read() {
        x1.as_pointer()
    } else {
        x2.as_pointer()
    }
    .read());
    let mut r2: i32 = (if x3.as_pointer().read() <= x4.as_pointer().read() {
        x3.as_pointer()
    } else {
        x4.as_pointer()
    }
    .read());
    let mut r3: i32 = (if (p1).clone().read() >= x2.as_pointer().read() {
        (p1).clone()
    } else {
        x2.as_pointer()
    }
    .read());
    let mut r4: i32 = (if (p2).clone().read() <= x3.as_pointer().read() {
        (p2).clone()
    } else {
        x3.as_pointer()
    }
    .read());
    let mut r5: i32 = {
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(30));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(40));
        (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    };
    assert!((((((r1 + r2) + r3) + r4) + r5) == 56));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
