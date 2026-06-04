extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static value_0: Value<i32> = Rc::new(RefCell::new(5));
);
pub fn param_shadow_1(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) + 1);
}
pub fn local_shadow_2() -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(99));
    return (*value.borrow());
}
pub fn read_global_3() -> i32 {
    return (*value_0.with(Value::clone).borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ param_shadow_1(10,) }) == 11) as i32) != 0));
    assert!((((({ local_shadow_2() }) == 99) as i32) != 0));
    assert!((((({ read_global_3() }) == 5) as i32) != 0));
    return 0;
}
