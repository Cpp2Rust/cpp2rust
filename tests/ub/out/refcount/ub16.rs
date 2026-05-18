extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a: Ptr<i32>) -> Ptr<i32> {
    let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
    return ((*a.borrow()).offset((5) as isize));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..10_u64)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    let out: Value<i32> = Rc::new(RefCell::new(
        (({
            let _a: Ptr<i32> = ((*p1.borrow()).offset((1) as isize));
            foo_0(_a)
        })
        .offset((4) as isize)
        .read()),
    ));
    (*p1.borrow()).delete_array();
    return 0;
}
