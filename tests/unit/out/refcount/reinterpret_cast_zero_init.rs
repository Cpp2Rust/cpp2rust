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
    let val: Value<u32> = Rc::new(RefCell::new(0_u32));
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new((val.as_pointer()).reinterpret_cast::<u8>()));
    (*bytes.borrow()).offset((0) as isize).write(239_u8);
    (*bytes.borrow()).offset((1) as isize).write(190_u8);
    (*bytes.borrow()).offset((2) as isize).write(173_u8);
    (*bytes.borrow()).offset((3) as isize).write(222_u8);
    assert!(((*val.borrow()) == 3735928559_u32));
    return 0;
}
