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
    let p: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::alloc(67305985_u32)));
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*p.borrow()).reinterpret_cast::<u8>()).clone(),
    ));
    assert!(((((*bytes.borrow()).offset((0) as isize).read()) as i32) == 1));
    assert!(((((*bytes.borrow()).offset((1) as isize).read()) as i32) == 2));
    assert!(((((*bytes.borrow()).offset((2) as isize).read()) as i32) == 3));
    assert!(((((*bytes.borrow()).offset((3) as isize).read()) as i32) == 4));
    (*bytes.borrow()).offset((0) as isize).write(16_u8);
    assert!((((*p.borrow()).read()) == 67306000_u32));
    (*p.borrow()).delete();
    return 0;
}
