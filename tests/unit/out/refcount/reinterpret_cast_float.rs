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
    let f: Value<f32> = Rc::new(RefCell::new(1.0E+0));
    let bits: Value<Ptr<u32>> = Rc::new(RefCell::new((f.as_pointer()).reinterpret_cast::<u32>()));
    assert!((((*bits.borrow()).read()) == 1065353216_u32));
    (*bits.borrow()).write(1073741824_u32);
    assert!(((*f.borrow()) == 2.0E+0));
    return 0;
}
