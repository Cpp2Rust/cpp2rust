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
    let b: Value<u8> = Rc::new(RefCell::new(66_u8));
    let p: Value<Ptr<u32>> = Rc::new(RefCell::new((b.as_pointer()).reinterpret_cast::<u32>()));
    let val: Value<u32> = Rc::new(RefCell::new(((*p.borrow()).read())));
    return 0;
}
