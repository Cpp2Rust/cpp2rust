extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        {
            let a0_clone = i.clone();
            (*v.borrow_mut()).push(a0_clone)
        };
        i.prefix_inc();
    }
    let mut sum: i32 = 0;
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut x: i32 = ((*__begin1.borrow()).iter_deref().read());
        sum += x;
        (*__begin1.borrow_mut()).iter_inc();
    }
    assert!((sum == 45));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
