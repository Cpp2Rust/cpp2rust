extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a: Ptr<i32>) -> Ptr<i32> {
    return (a).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let pa: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    let b: Ptr<i32> = ({
        let _a: Ptr<i32> = (*pa.borrow()).clone();
        foo_0(_a)
    });
    return (b.read());
}
