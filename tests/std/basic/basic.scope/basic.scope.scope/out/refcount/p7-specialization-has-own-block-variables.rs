extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn next_id_0() -> i32 {
    return (*last_1.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn next_id_2() -> i32 {
    return (*last_3.with(Value::clone).borrow_mut()).prefix_inc();
}
thread_local!(
    static last_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static last_3: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ next_id_0() }) == 1));
    assert!((({ next_id_0() }) == 2));
    assert!((({ next_id_2() }) == 1));
    assert!((({ next_id_0() }) == 3));
    assert!((({ next_id_2() }) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
