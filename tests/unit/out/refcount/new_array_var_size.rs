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
    let N: Value<i32> = Rc::new(RefCell::new(5));
    let A: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..((*N.borrow()) as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    (*A.borrow()).delete_array();
    let N2: Ptr<i32> = N.as_pointer();
    let A2: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..((N2.read()) as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    (*A2.borrow()).delete_array();
    return 0;
}
