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
    let h: Value<i32> = Rc::new(RefCell::new(15));
    let h_ref1: Ptr<i32> = h.as_pointer();
    h_ref1.write(16);
    let h_ptr: Value<Ptr<i32>> = Rc::new(RefCell::new((h_ref1).clone()));
    let h_ref2: Ptr<i32> = (*h_ptr.borrow()).clone();
    h_ref2.write(17);
    return {
        let _lhs = (h_ref1.read());
        _lhs + (h_ref2.read())
    };
}
