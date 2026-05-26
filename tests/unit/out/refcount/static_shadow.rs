extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static s_value: Value<i32> = Rc::new(RefCell::new(5));
);
pub fn param_shadow_0(value: i32) -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) + 1);
}
pub fn local_shadow_1() -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(99));
    return (*value.borrow());
}
pub fn read_global_2() -> i32 {
    return (*s_value.with(Value::clone).borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _value: i32 = 10;
            param_shadow_0(_value)
        }) == 11) as i32)
            != 0)
    );
    assert!((((({ local_shadow_1() }) == 99) as i32) != 0));
    assert!((((({ read_global_2() }) == 5) as i32) != 0));
    return 0;
}
