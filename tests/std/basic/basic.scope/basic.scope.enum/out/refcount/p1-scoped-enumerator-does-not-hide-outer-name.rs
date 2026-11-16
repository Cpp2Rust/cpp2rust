extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Power = i32;
pub const Power_off: Power = 3;
pub const Power_on: Power = 8;
thread_local!(
    pub static off_0: Value<i32> = Rc::new(RefCell::new(40));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((off_0.with(|rc| *rc.borrow()) == 40));
    assert!(((Power_off as i32) == 3));
    off_0.with(|rc| *rc.borrow_mut() += (Power_on as i32));
    assert!((off_0.with(|rc| *rc.borrow()) == 48));
    assert!(((Power_off as i32) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = off_0.with(|_| ());
}
