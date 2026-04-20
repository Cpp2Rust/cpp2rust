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
    let v: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    let bytes: Value<Ptr<u16>> = Rc::new(RefCell::new(
        (v.as_pointer() as Ptr<u32>).reinterpret_cast::<u16>(),
    ));
    return 0;
}
