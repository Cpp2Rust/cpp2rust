extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static version_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn version_2() -> i32 {
    (*version_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return 2;
}
thread_local!(
    pub static revision_1: Value<i32> = Rc::new(RefCell::new(40));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ version_2() }) == 2));
    assert!((version_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((({ version_2() }) == 2));
    assert!((version_calls_0.with(|rc| *rc.borrow()) == 2));
    assert!(((revision_1.with(|v| v.as_pointer())) == (revision_1.with(|v| v.as_pointer()))));
    revision_1.with(|rc| *rc.borrow_mut() = 41);
    assert!((revision_1.with(|rc| *rc.borrow()) == 41));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = version_calls_0.with(|_| ());
    let _ = revision_1.with(|_| ());
}
