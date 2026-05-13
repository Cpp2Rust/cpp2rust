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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        120_u8, 86_u8, 52_u8, 18_u8, 239_u8, 205_u8, 171_u8, 144_u8,
    ])));
    let dwords: Value<Ptr<u32>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<u8>).reinterpret_cast::<u32>(),
    ));
    assert!((((*dwords.borrow()).offset((0) as isize).read()) == 305419896_u32));
    assert!((((*dwords.borrow()).offset((1) as isize).read()) == 2427178479_u32));
    return 0;
}
