extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    static __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
    pub static g_0: Ptr<i32> = __tmp_0.with(Value::as_pointer);
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((g_0.with(Ptr::clone).read()) == 5));
    return 0;
}
