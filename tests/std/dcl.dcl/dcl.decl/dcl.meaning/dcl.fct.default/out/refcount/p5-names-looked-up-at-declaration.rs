extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static scaled_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static rate_1: Value<i32> = Rc::new(RefCell::new(1));
);
pub fn scaled_2(mut amount: i32) -> i32 {
    (*scaled_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (amount * 10);
}
pub fn total_3(base: Option<i32>) -> i32 {
    let mut base: i32 = base.unwrap_or_else(|| ({ scaled_2(rate_1.with(|rc| *rc.borrow())) }));
    return base;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    rate_1.with(|rc| *rc.borrow_mut() = 2);
    {
        let mut rate: i32 = 3;
        assert!((({ total_3(None,) }) == 20));
        assert!((scaled_calls_0.with(|rc| *rc.borrow()) == 1));
        assert!((rate == 3));
    }
    assert!((({ total_3(Some(5),) }) == 5));
    assert!((scaled_calls_0.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = scaled_calls_0.with(|_| ());
    let _ = rate_1.with(|_| ());
}
