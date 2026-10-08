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
    let i1: Value<i32> = Rc::new(RefCell::new(42));
    let mut ptr1: Ptr<i32> = (i1.as_pointer());
    let mut ptr2: Ptr<i8> = (i1.as_pointer()).reinterpret_cast::<i8>();
    let mut vptr1: AnyPtr = (ptr1).to_any();
    let mut vptr2: AnyPtr = (ptr2).to_any();
    assert!(({ (vptr1).clone() } == { (vptr2).clone() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
