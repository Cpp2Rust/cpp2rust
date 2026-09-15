extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static same_name_different_type_0: Value<i32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static same_name_same_type_1: Value<i32> = Rc::new(RefCell::new(5));
);
pub fn a_foo_2() -> i32 {
    return (*same_name_different_type_0.with(Value::clone).borrow());
}
pub fn a_bar_3() -> i32 {
    return (*same_name_same_type_1.with(Value::clone).borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ a_foo_2() }) == 1) as i32) != 0));
    assert!((((({ b_foo_4() }) == 1.0E+0) as i32) != 0));
    assert!((((({ a_bar_3() }) == 5) as i32) != 0));
    assert!((((({ b_bar_5() }) == 6) as i32) != 0));
    return 0;
}
thread_local!(
    pub static same_name_different_type_6: Value<f32> = Rc::new(RefCell::new(1.0E+0));
);
thread_local!(
    pub static same_name_same_type_7: Value<i32> = Rc::new(RefCell::new(6));
);
pub fn b_foo_4() -> f32 {
    return (*same_name_different_type_6.with(Value::clone).borrow());
}
pub fn b_bar_5() -> i32 {
    return (*same_name_same_type_7.with(Value::clone).borrow());
}
pub fn __cpp2rust_init_globals() {
    let _ = same_name_different_type_0.with(Value::clone);
    let _ = same_name_same_type_1.with(Value::clone);
    let _ = same_name_different_type_6.with(Value::clone);
    let _ = same_name_same_type_7.with(Value::clone);
}
