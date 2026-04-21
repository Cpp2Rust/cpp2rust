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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let r: Ptr<i32> = a.as_pointer();
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    r.write(2);
    (*p.borrow()).write(3);
    let p2: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new((p.as_pointer())));
    ((*p2.borrow()).read()).write(3);
    return 0;
}
