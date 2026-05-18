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
    let a: Ptr<i32> = ({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
        if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
    });
    let b: Ptr<i32> = ({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
        if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
    });
    return (({
        let _lhs = (a.read());
        _lhs == (b.read())
    }) as i32);
}
