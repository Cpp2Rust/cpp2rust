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
    let vals: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
        (-1_i32 as i16),
        256_i16,
        (-32768_i32 as i16),
    ])));
    let uvals: Value<Ptr<u16>> = Rc::new(RefCell::new(
        (vals.as_pointer() as Ptr<i16>).reinterpret_cast::<u16>(),
    ));
    assert!(((((*uvals.borrow()).offset((0) as isize).read()) as i32) == 65535));
    assert!(((((*uvals.borrow()).offset((1) as isize).read()) as i32) == 256));
    assert!(((((*uvals.borrow()).offset((2) as isize).read()) as i32) == 32768));
    (*uvals.borrow()).offset((0) as isize).write(42_u16);
    assert!((((*vals.borrow())[(0) as usize] as i32) == 42));
    return 0;
}
