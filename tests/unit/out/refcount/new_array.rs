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
    let array: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..100_u64)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    (*array.borrow()).delete_array();
    return 0;
}
