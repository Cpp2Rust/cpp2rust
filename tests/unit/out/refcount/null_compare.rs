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
    let a: Value<i32> = Rc::new(RefCell::new(0));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    assert!((((!((*p.borrow()).is_null())) as i32) != 0));
    assert!((((!((*p.borrow()).is_null())) as i32) != 0));
    (*p.borrow_mut()) = Ptr::<i32>::null();
    assert!(((((*p.borrow()).is_null()) as i32) != 0));
    assert!(((((*p.borrow()).is_null()) as i32) != 0));
    return 0;
}
