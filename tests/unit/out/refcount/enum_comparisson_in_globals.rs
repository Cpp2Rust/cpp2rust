extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type E = u32;
pub const E_A: E = 0;
pub const E_B: E = 1;
thread_local!(
    pub static global_0: Value<i32> =
        Rc::new(RefCell::new((((E_A as i32) != (E_B as i32)) as i32)));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((*global_0.with(Value::clone).borrow()) == 1));
    return 0;
}
