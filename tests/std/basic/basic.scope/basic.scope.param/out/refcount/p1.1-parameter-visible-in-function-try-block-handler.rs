extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static body_runs_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static handler_runs_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn recover_2(mut fallback: i32) -> i32 {
    {
        (*body_runs_0.with(Value::clone).borrow_mut()).prefix_inc();
        8;
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ recover_2(30,) }) == 38));
    assert!((body_runs_0.with(|rc| *rc.borrow()) == 1));
    assert!((handler_runs_1.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = body_runs_0.with(|_| ());
    let _ = handler_runs_1.with(|_| ());
}
