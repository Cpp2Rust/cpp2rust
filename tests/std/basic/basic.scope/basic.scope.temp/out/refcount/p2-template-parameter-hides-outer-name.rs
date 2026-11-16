extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static width_0: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn area_1(mut height: i32) -> i32 {
    return (5 * height);
}
pub fn area_2(mut height: i32) -> i32 {
    return (7 * height);
}
pub fn plain_area_3(mut height: i32) -> i32 {
    return (width_0.with(|rc| *rc.borrow()) * height);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ area_1(2,) }) == 10));
    assert!((({ area_2(2,) }) == 14));
    assert!((({ plain_area_3(2,) }) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = width_0.with(|_| ());
}
