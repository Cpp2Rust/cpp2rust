extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<u32> = Rc::new(RefCell::new(42_u32));
    assert!((((((a.as_pointer()).to_any().reinterpret_cast::<i32>().read()) == 42) as i32) != 0));
    return 0;
}
