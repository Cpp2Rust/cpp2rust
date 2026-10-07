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
    let mut out: i32 = 0;
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 0])));
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(0));
    'loop_: while ((ptr.read()) != 0) {
        out += { (ptr.read()) };
        ptr.prefix_inc();
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(1));
    'loop_: while ((ptr.read()) != 4) {
        out += { (ptr.read()) };
        ptr.postfix_inc();
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(4));
    'loop_: while ((ptr.read()) != 1) {
        out += { (ptr.read()) };
        ptr.postfix_dec();
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(3));
    'loop_: while ((ptr.read()) != 2) {
        out += { (ptr.read()) };
        ptr.prefix_dec();
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(0));
    'loop_: while ((ptr.read()) != 0) {
        out += { (ptr.read()) };
        ptr = { ptr.offset((1) as isize) };
    }
    let mut ptr: Ptr<i32> = ((arr.as_pointer() as Ptr<i32>).offset(0));
    let mut i: i32 = 0;
    'loop_: while (i < 5) {
        out += { (elem!(ptr, i).read()) };
        i.prefix_inc();
    }
    assert!((out == 51));
    let mut step: i64 = 2_i64;
    let mut offset: Ptr<i32> = (arr.as_pointer() as Ptr<i32>).offset((step) as isize);
    assert!(({ (offset.read()) } == { (*arr.borrow())[(2) as usize] }));
    offset = { offset.offset(-((step) as isize)) };
    assert!(({ (offset).clone() } == { ((arr.as_pointer() as Ptr<i32>).offset(0)) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
