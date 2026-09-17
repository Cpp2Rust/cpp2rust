extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn a_0() -> i32 {
    thread_local!(
        static i_1: Value<i32> = Rc::new(RefCell::new(1));
    );
    return i_1.with(|rc| rc.borrow().clone());
}
pub fn b_2() -> i32 {
    thread_local!(
        static i_3: Value<i32> = Rc::new(RefCell::new(2));
    );
    return i_3.with(|rc| rc.borrow().clone());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ a_0() }) == 1));
    assert!((({ b_2() }) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
