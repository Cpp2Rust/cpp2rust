extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f1_0(first: Ptr<i32>, last: Ptr<i32>) {
    let first: Value<Ptr<i32>> = Rc::new(RefCell::new(first));
    let last: Value<Ptr<i32>> = Rc::new(RefCell::new(last));
    (*first.borrow())
        .clone()
        .sort((*last.borrow()).clone().get_offset());
    return;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 3, 0, 2, 8, 7]));
    ({
        let _first: Ptr<i32> = (v.as_pointer() as Ptr<i32>);
        let _last: Ptr<i32> = (v.as_pointer() as Ptr<i32>).to_end();
        f1_0(_first, _last)
    });
    return 0;
}
