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
    let neg: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let words: Value<Ptr<u16>> =
        Rc::new(RefCell::new((neg.as_pointer()).reinterpret_cast::<u16>()));
    assert!(((((*words.borrow()).offset((0) as isize).read()) as i32) == 65535));
    assert!(((((*words.borrow()).offset((1) as isize).read()) as i32) == 65535));
    let neg64: Value<i64> = Rc::new(RefCell::new((-256_i32 as i64)));
    let quarters: Value<Ptr<i16>> =
        Rc::new(RefCell::new((neg64.as_pointer()).reinterpret_cast::<i16>()));
    assert!(((((*quarters.borrow()).offset((0) as isize).read()) as i32) == -256_i32));
    assert!(((((*quarters.borrow()).offset((1) as isize).read()) as i32) == -1_i32));
    assert!(((((*quarters.borrow()).offset((2) as isize).read()) as i32) == -1_i32));
    assert!(((((*quarters.borrow()).offset((3) as isize).read()) as i32) == -1_i32));
    return 0;
}
