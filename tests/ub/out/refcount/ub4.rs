extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn smaller_0(x1: Ptr<i32>, x2: Ptr<i32>) -> Ptr<i32> {
    return if ({ (x1.read()) } < { (x2.read()) }) {
        (x1).clone()
    } else {
        (x2).clone()
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut out: Ptr<i32> = Ptr::<i32>::null();
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    if ((*x1.borrow()) != 0) {
        let x2: Value<i32> = Rc::new(RefCell::new(-1_i32));
        out = ({ smaller_0(x1.as_pointer(), x2.as_pointer()) });
    }
    return (out.read());
}
pub fn __cpp2rust_init_globals() {}
