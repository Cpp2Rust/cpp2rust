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
    let rows: Value<Box<[Box<[i32]>]>> = Rc::new(RefCell::new(Box::new([
        Box::new([1, 2, 3]),
        Box::new([4, 5, 6]),
    ])));
    return ((((((rows.as_pointer() as Ptr<Box<[i32]>>)
        .offset(1)
        .as_pointer()) as Ptr<i32>)
        - (((((rows.as_pointer() as Ptr<Box<[i32]>>)
            .offset(0)
            .as_pointer()) as Ptr<i32>)
            .offset(0))
        .offset((2) as isize))) as i64) as i32);
}
pub fn __cpp2rust_init_globals() {}
