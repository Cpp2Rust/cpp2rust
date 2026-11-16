extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static value_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static pointer_calls_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn describe_2(mut value: i32) -> i32 {
    (*value_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (value + 10);
}
pub fn describe_3(mut pointer: Ptr<i32>) -> i32 {
    (*pointer_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    return ((pointer.read()) + 20);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let amount: Value<i32> = Rc::new(RefCell::new(3));
    assert!(({ (({ describe_2((*amount.borrow()),) }) == 13) }));
    assert!((value_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((pointer_calls_1.with(|rc| *rc.borrow()) == 0));
    assert!((({ describe_3((amount.as_pointer()),) }) == 23));
    assert!((value_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((pointer_calls_1.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = value_calls_0.with(|_| ());
    let _ = pointer_calls_1.with(|_| ());
}
