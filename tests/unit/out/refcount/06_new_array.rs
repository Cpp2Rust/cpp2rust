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
    let e: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..2_u64).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    )));
    (*e.borrow()).offset((0) as isize).write(6);
    (*e.borrow()).offset((1) as isize).write(7);
    (*e.borrow()).delete_array();
    return 0;
}
