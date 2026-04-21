extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(single: Ptr<i32>) {
    let single: Value<Ptr<i32>> = Rc::new(RefCell::new(single));
    (*single.borrow()).delete();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..10_u64)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    ({
        let _single: Ptr<i32> = (*x.borrow()).clone();
        foo_0(_single)
    });
    return 0;
}
