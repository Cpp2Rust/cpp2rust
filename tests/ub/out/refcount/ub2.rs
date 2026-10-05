extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn null_0() -> Ptr<i32> {
    let mut p: Ptr<i32> = Ptr::<i32>::null();
    return p;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: Ptr<i32> = ({ null_0() });
    return (x.read());
}
pub fn __cpp2rust_init_globals() {}
