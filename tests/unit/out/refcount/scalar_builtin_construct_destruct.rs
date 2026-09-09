extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn zero_0() -> Ptr<i32> {
    return Ptr::<i32>::null();
}
pub fn zero_1() -> i64 {
    return <i64>::default();
}
pub fn destroy_2(p: Ptr<i32>) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<i32> = Rc::new(RefCell::new(<i32>::default()));
    let d: Value<f64> = Rc::new(RefCell::new(<f64>::default()));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(({ zero_0() })));
    assert!(((*i.borrow()) == 0));
    assert!(((*d.borrow()) == 0.0E+0));
    assert!((*p.borrow()).is_null());
    assert!((({ zero_1() }) == 0_i64));
    let x: Value<i32> = Rc::new(RefCell::new(5));
    ({ destroy_2((x.as_pointer())) });
    assert!(((*x.borrow()) == 5));
    return 0;
}
