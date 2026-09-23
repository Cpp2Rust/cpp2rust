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
    let arr1: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..100_usize).map(|_| 0_i32).collect::<Box<[i32]>>(),
    )));
    (*arr1.borrow()).offset((100) as isize).write(1);
    (*arr1.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
