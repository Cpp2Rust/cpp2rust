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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut measure: i32 = 5;
    {
        assert!((({ measure_1() }) == 7));
        assert!((function_calls_0.with(|rc| *rc.borrow()) == 1));
    }
    assert!((measure == 5));
    return 0;
}
pub fn measure_1() -> i32 {
    (*function_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return 7;
}
pub fn __cpp2rust_init_globals() {
    let _ = function_calls_0.with(|_| ());
}
