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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..10_usize).map(|_| 0_i32).collect::<Box<[i32]>>(),
    )));
    ({ foo_0((*x.borrow()).clone()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
