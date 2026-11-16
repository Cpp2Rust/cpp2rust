extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static rate_0: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn scale_1(mut rate: i32) -> i32 {
    rate *= 2;
    return rate;
}
pub fn read_rate_2() -> i32 {
    return rate_0.with(|rc| *rc.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ scale_1(5,) }) == 10));
    assert!((rate_0.with(|rc| *rc.borrow()) == 3));
    assert!((({ read_rate_2() }) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = rate_0.with(|_| ());
}
