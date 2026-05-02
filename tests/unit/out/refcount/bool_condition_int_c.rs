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
    let n: Value<i32> = Rc::new(RefCell::new(3));
    let zero: Value<i32> = Rc::new(RefCell::new(0));
    let u: Value<u32> = Rc::new(RefCell::new(4_u32));
    let ul: Value<u64> = Rc::new(RefCell::new(5_u64));
    let ll: Value<i64> = Rc::new(RefCell::new(6_i64));
    let ch: Value<u8> = Rc::new(RefCell::new((('a' as i32) as u8)));
    if ((*n.borrow()) != 0) {
        assert!((1 != 0));
    }
    if !((*n.borrow()) != 0) {
        assert!((0 != 0));
    }
    if ((*zero.borrow()) != 0) {
        assert!((0 != 0));
    }
    if !((*zero.borrow()) != 0) {
        assert!((1 != 0));
    }
    if ((*u.borrow()) != 0) {
        assert!((1 != 0));
    }
    if ((*ul.borrow()) != 0) {
        assert!((1 != 0));
    }
    if ((*ll.borrow()) != 0) {
        assert!((1 != 0));
    }
    if ((*ch.borrow()) != 0) {
        assert!((1 != 0));
    }
    let loop_count: Value<i32> = Rc::new(RefCell::new(0));
    let counter: Value<i32> = Rc::new(RefCell::new(3));
    'loop_: while ((*counter.borrow()) != 0) {
        (*counter.borrow_mut()).prefix_dec();
        (*loop_count.borrow_mut()).prefix_inc();
    }
    assert!(((*loop_count.borrow()) == 3));
    let i: Value<i32> = Rc::new(RefCell::new(5));
    'loop_: while ((*i.borrow()) != 0) {
        (*loop_count.borrow_mut()).prefix_inc();
        (*i.borrow_mut()).prefix_dec();
    }
    assert!(((*loop_count.borrow()) == 8));
    let t: Value<i32> = Rc::new(RefCell::new(if ((*n.borrow()) != 0) { 100 } else { 200 }));
    assert!(((*t.borrow()) == 100));
    let t2: Value<i32> = Rc::new(RefCell::new(if ((*zero.borrow()) != 0) {
        100
    } else {
        200
    }));
    assert!(((*t2.borrow()) == 200));
    let t7: Value<i32> = Rc::new(RefCell::new((!((*n.borrow()) != 0) as i32)));
    assert!(((*t7.borrow()) == 0));
    let t8: Value<i32> = Rc::new(RefCell::new((!((*zero.borrow()) != 0) as i32)));
    assert!(((*t8.borrow()) == 1));
    let b1: Value<bool> = Rc::new(RefCell::new(((*n.borrow()) != 0)));
    assert!((*b1.borrow()));
    return 0;
}
