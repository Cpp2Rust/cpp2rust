extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static function_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Reading {
    #[offset(0)]
    pub value: i32,
}
pub fn Reading_1(mut raw_: i32) -> i32 {
    (*function_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (raw_ + 1);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut reading: Reading = Reading { value: 4 };
    assert!((reading.value == 4));
    assert!((function_calls_0.with(|rc| *rc.borrow()) == 0));
    assert!((({ Reading_1(reading.value,) }) == 5));
    assert!((function_calls_0.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = function_calls_0.with(|_| ());
}
