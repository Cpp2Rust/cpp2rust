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
pub fn test_1() {
    let mut code: i32 = 1;
    {
        2;
    };
    assert!((handler_runs_0.with(|rc| *rc.borrow()) == 1));
    assert!((code == 1));
}
thread_local!(
    pub static handler_runs_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn test_3() {
    let mut stage: i32 = 10;
    {
        let mut stage: i32 = 20;
        stage;
    };
    assert!((handler_runs_2.with(|rc| *rc.borrow()) == 1));
    assert!((stage == 10));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    ({ test_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = handler_runs_0.with(|_| ());
    let _ = handler_runs_2.with(|_| ());
}
