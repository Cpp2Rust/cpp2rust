extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn second_row_0() -> Ptr<i32> {
    let rows: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::from([
        Vec::from([1, 2, 3]),
        Vec::from([4, 5, 6]),
    ])));
    return ((rows.as_pointer() as Ptr<Vec<i32>>)
        .offset(1_usize)
        .as_pointer() as Ptr<Vec<i32>>)
        .decay();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<i32> = ({ second_row_0() });
    return (elem!(p, 0).read());
}
pub fn __cpp2rust_init_globals() {}
