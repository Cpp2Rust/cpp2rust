extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first_0: Value<i32> = Rc::new(RefCell::new(100));
);
pub type Step = i32;
pub const Step_first: Step = 2;
pub const Step_second: Step = 10;
pub type Stride = i32;
pub const Stride_single: Stride = 101;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((Step_first as i32) == 2));
    assert!(((Step_second as i32) == 10));
    assert!(((Stride_single as i32) == 101));
    assert!((first_0.with(|rc| *rc.borrow()) == 100));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = first_0.with(|_| ());
}
