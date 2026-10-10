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
    let neg: Value<i32> = Rc::new(RefCell::new(-1_i32));
    let mut words: Ptr<u16> = (neg.as_pointer()).reinterpret_cast::<u16>();
    assert!((((elem!(words, 0).read()) as i32) == 65535));
    assert!((((elem!(words, 1).read()) as i32) == 65535));
    let neg64: Value<i64> = Rc::new(RefCell::new((-256_i32 as i64)));
    let mut quarters: Ptr<i16> = (neg64.as_pointer()).reinterpret_cast::<i16>();
    assert!((((elem!(quarters, 0).read()) as i32) == -256_i32));
    assert!((((elem!(quarters, 1).read()) as i32) == -1_i32));
    assert!((((elem!(quarters, 2).read()) as i32) == -1_i32));
    assert!((((elem!(quarters, 3).read()) as i32) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
