extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(4));
);
thread_local!(
    pub static count_1: Value<i32> = Rc::new(RefCell::new(100));
);
pub fn next_2() -> i32 {
    return (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ next_2() }) == 5));
    assert!((count_0.with(|rc| *rc.borrow()) == 5));
    assert!((count_1.with(|rc| *rc.borrow()) == 100));
    assert!((count_1.with(|rc| *rc.borrow()) == 100));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = count_1.with(|_| ());
}
