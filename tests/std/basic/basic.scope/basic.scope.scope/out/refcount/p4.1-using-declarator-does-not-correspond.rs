extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static library_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static local_calls_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn describe_2(mut amount: i32) -> i32 {
    (*library_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (amount + 10);
}
pub fn describe_3(mut amount: f64) -> i32 {
    (*local_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    return ((amount as i32) + 20);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ describe_2(1,) }) == 11));
    assert!((library_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((local_calls_1.with(|rc| *rc.borrow()) == 0));
    assert!((({ describe_3(2_f64,) }) == 22));
    assert!((library_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((local_calls_1.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = library_calls_0.with(|_| ());
    let _ = local_calls_1.with(|_| ());
}
