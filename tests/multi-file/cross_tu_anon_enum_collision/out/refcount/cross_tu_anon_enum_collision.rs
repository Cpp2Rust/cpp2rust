extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_ALPHA: anon_0 = 7;
pub fn a_value_1() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    (*x.borrow_mut()) |= (anon_0_ALPHA as i32);
    return (*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ a_value_1() }) == 7) as i32) != 0));
    assert!((((({ b_value_2() }) == 9) as i32) != 0));
    return 0;
}
pub type anon_3 = u32;
pub const anon_3_BETA: anon_3 = 9;
pub fn b_value_2() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    (*x.borrow_mut()) |= (anon_3_BETA as i32);
    return (*x.borrow());
}
