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
    let value: Value<u32> = Rc::new(RefCell::new(67305985_u32));
    let bytes: Value<Ptr<u8>> =
        Rc::new(RefCell::new((value.as_pointer()).reinterpret_cast::<u8>()));
    assert!(((((*bytes.borrow()).offset((0) as isize).read()) as i32) == 1));
    assert!(((((*bytes.borrow()).offset((1) as isize).read()) as i32) == 2));
    assert!(((((*bytes.borrow()).offset((2) as isize).read()) as i32) == 3));
    assert!(((((*bytes.borrow()).offset((3) as isize).read()) as i32) == 4));
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8, 4_u8])));
    let arr16: Value<Ptr<u16>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<u8>).reinterpret_cast::<u16>(),
    ));
    assert!(((((*arr16.borrow()).offset((0) as isize).read()) as i32) == 513));
    assert!(((((*arr16.borrow()).offset((1) as isize).read()) as i32) == 1027));
    return 0;
}
