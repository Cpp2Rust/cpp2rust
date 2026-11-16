extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static level_0: Value<i32> = Rc::new(RefCell::new(11));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut level: i32 = 22;
    assert!((level == 22));
    {
        assert!((level_0.with(|rc| *rc.borrow()) == 11));
        level_0.with(|rc| *rc.borrow_mut() = 33);
    }
    assert!((level == 22));
    assert!((level_0.with(|rc| *rc.borrow()) == 33));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = level_0.with(|_| ());
}
