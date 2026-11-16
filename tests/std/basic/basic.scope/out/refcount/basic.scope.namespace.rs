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
pub fn helper_2() -> i32 {
    (*global_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return 100;
}
pub fn run_3() -> i32 {
    return ({ helper_4() });
}
pub fn helper_4() -> i32 {
    (*inner_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    return 9;
}
pub fn test_5() {
    assert!((({ run_3() }) == 9));
    assert!((inner_calls_1.with(|rc| *rc.borrow()) == 1));
    assert!((global_calls_0.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static global_calls_6: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static inner_calls_7: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn base_8() -> i32 {
    (*global_calls_6.with(Value::clone).borrow_mut()).prefix_inc();
    return 100;
}
pub fn base_9() -> i32 {
    (*inner_calls_7.with(Value::clone).borrow_mut()).prefix_inc();
    return 7;
}
pub fn twice_11() -> i32 {
    return (({ base_9() }) * 2);
}
thread_local!(
    pub static stored_10: Value<i32> = Rc::new(RefCell::new((({ base_9() }) + 1)));
);
pub fn test_12() {
    assert!((stored_10.with(|rc| *rc.borrow()) == 8));
    assert!((inner_calls_7.with(|rc| *rc.borrow()) == 1));
    assert!((({ twice_11() }) == 14));
    assert!((inner_calls_7.with(|rc| *rc.borrow()) == 2));
    assert!((global_calls_6.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static count_13: Value<i32> = Rc::new(RefCell::new(4));
);
thread_local!(
    pub static count_14: Value<i32> = Rc::new(RefCell::new(100));
);
pub fn next_15() -> i32 {
    return (*count_13.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn test_16() {
    assert!((({ next_15() }) == 5));
    assert!((count_13.with(|rc| *rc.borrow()) == 5));
    assert!((count_14.with(|rc| *rc.borrow()) == 100));
    assert!((count_14.with(|rc| *rc.borrow()) == 100));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_5() });
    ({ test_12() });
    ({ test_16() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = global_calls_0.with(|_| ());
    let _ = inner_calls_1.with(|_| ());
    let _ = global_calls_6.with(|_| ());
    let _ = inner_calls_7.with(|_| ());
    let _ = stored_10.with(|_| ());
    let _ = count_13.with(|_| ());
    let _ = count_14.with(|_| ());
}
