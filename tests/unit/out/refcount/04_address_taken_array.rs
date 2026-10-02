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
    let arr2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 2])));
    (*arr2.borrow_mut())[(0) as usize] = 3;
    (*arr2.borrow_mut())[(1) as usize] = 4;
    let arr2_ptr: Value<Ptr<i32>> = Rc::new(RefCell::new((arr2.as_pointer() as Ptr<i32>)));
    elem!((*arr2_ptr.borrow()), 0).write(5);
    elem!((*arr2_ptr.borrow()), 1).write(6);
    let arr2_ref1: Ptr<i32> = (arr2.as_pointer() as Ptr<i32>).offset(1);
    arr2_ref1.write(7);
    assert!((((*arr2.borrow())[(0) as usize] + (*arr2.borrow())[(1) as usize]) == 12));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
