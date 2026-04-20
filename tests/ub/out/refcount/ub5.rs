extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn null_0(p: Ptr<Ptr<i32>>) {
    let p: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new(p));
    (*p.borrow()).write(Default::default());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((x.as_pointer())));
    ({
        let _p: Ptr<Ptr<i32>> = (p.as_pointer());
        null_0(_p)
    });
    let r: Ptr<i32> = (*p.borrow()).clone();
    return (r.read());
}
