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
    let arr: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..2_u64).map(|_| <u32>::default()).collect::<Box<[u32]>>(),
    )));
    (*arr.borrow()).offset((0) as isize).write(67305985_u32);
    (*arr.borrow()).offset((1) as isize).write(134678021_u32);
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*arr.borrow()).reinterpret_cast::<u8>()).clone(),
    ));
    assert!(((((*bytes.borrow()).offset((0) as isize).read()) as i32) == 1));
    assert!(((((*bytes.borrow()).offset((4) as isize).read()) as i32) == 5));
    assert!(((((*bytes.borrow()).offset((7) as isize).read()) as i32) == 8));
    (*bytes.borrow()).offset((0) as isize).write(170_u8);
    assert!((((*arr.borrow()).offset((0) as isize).read()) == 67306154_u32));
    (*bytes.borrow()).offset((5) as isize).write(187_u8);
    assert!((((*arr.borrow()).offset((1) as isize).read()) == 134724357_u32));
    (*arr.borrow()).delete_array();
    return 0;
}
