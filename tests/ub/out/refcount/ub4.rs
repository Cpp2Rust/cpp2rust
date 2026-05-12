extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn smaller_0(x1: Ptr<i32>, x2: Ptr<i32>) -> Ptr<i32> {
    return if ({
        let _lhs = (x1.read());
        _lhs < (x2.read())
    }) {
        (x1).clone()
    } else {
        (x2).clone()
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let out: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    if ((*x1.borrow()) != 0) {
        let x2: Value<i32> = Rc::new(RefCell::new(-1_i32));
        (*out.borrow_mut()) = ({
            let _x1: Ptr<i32> = x1.as_pointer();
            let _x2: Ptr<i32> = x2.as_pointer();
            smaller_0(_x1, _x2)
        });
    }
    return ((*out.borrow()).read());
}
