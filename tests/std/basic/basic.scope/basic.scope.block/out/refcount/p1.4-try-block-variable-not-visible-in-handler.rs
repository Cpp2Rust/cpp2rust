extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static handler_runs_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut stage: i32 = 10;
    {
        let mut stage: i32 = 20;
        stage;
    };
    assert!((handler_runs_0.with(|rc| *rc.borrow()) == 1));
    assert!((stage == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = handler_runs_0.with(|_| ());
}
