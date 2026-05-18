extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a0: Ptr<i32>) {
    let a0: Value<Ptr<i32>> = Rc::new(RefCell::new(a0));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(
        (0..(4_u64) as usize)
            .map(|_| <i32>::default())
            .collect::<Vec<_>>(),
    ));
    let it: Value<Ptr<i32>> = Rc::new(RefCell::new((vec_.as_pointer() as Ptr<i32>)));
    ({
        let _a0: Ptr<i32> = (*it.borrow()).clone();
        foo_0(_a0)
    });
    return 0;
}
