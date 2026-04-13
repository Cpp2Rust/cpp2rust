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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2]));
    let b: Ptr<i32> = ({
        let _a: Ptr<i32> = (v.as_pointer() as Ptr<i32>);
        foo_0(_a)
    });
    (*v.borrow_mut()).clear();
    return (b.read());
}
