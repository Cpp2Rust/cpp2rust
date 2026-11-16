extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static global_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static inner_calls_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn base_2() -> i32 {
    (*global_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return 100;
}
pub fn base_3() -> i32 {
    (*inner_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    return 7;
}
pub fn twice_5() -> i32 {
    return (({ base_3() }) * 2);
}
thread_local!(
    pub static stored_4: Value<i32> = Rc::new(RefCell::new((({ base_3() }) + 1)));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((stored_4.with(|rc| *rc.borrow()) == 8));
    assert!((inner_calls_1.with(|rc| *rc.borrow()) == 1));
    assert!((({ twice_5() }) == 14));
    assert!((inner_calls_1.with(|rc| *rc.borrow()) == 2));
    assert!((global_calls_0.with(|rc| *rc.borrow()) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = global_calls_0.with(|_| ());
    let _ = inner_calls_1.with(|_| ());
    let _ = stored_4.with(|_| ());
}
