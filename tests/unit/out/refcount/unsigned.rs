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
    let x: Value<u32> = Rc::new(RefCell::new((-1_i32 as u32)));
    assert!(((*x.borrow()) == <u32>::MAX));
    let mut v1: i32 = (((*x.borrow()) & 1_u32) as i32);
    let mut v2: u32 = ((*x.borrow()) & 1_u32);
    let mut p: Ptr<u32> = (x.as_pointer());
    let mut b: bool = (((p.read()) & 255_u32) != 0);
    let mut a: i32 = (((p.read()) & 255_u32) as i32);
    assert!((a == 255));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
