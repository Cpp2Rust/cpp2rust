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
    let a: Value<u32> = Rc::new(RefCell::new(0_u32));
    let mut p: Ptr<u32> = (a.as_pointer());
    p.write({ (p.read()).wrapping_add((p.read())) });
    return ((p.read()) as i32);
}
pub fn __cpp2rust_init_globals() {}
