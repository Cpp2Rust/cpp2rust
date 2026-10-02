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
    let x1: Value<i32> = Rc::new(RefCell::new(-1_i32));
    assert!(((((*x1.borrow()) == -1_i32) as i32) != 0));
    let x2: Value<i8> = Rc::new(RefCell::new(-1_i8));
    assert!((((((*x2.borrow()) as i32) == -1_i32) as i32) != 0));
    let u1: Value<u32> = Rc::new(RefCell::new(5_u32));
    let u2: Value<u32> = Rc::new(RefCell::new((*u1.borrow()).wrapping_neg()));
    assert!(((((*u2.borrow()) == 4294967291_u32) as i32) != 0));
    let c1: Value<u8> = Rc::new(RefCell::new(255_u8));
    assert!(((((((*c1.borrow()) as u8) as i32) == 255) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
