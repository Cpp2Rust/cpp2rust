extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static factor_0: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn scaled_1(mut v: i32) -> i32 {
    return (v * factor_0.with(|rc| *rc.borrow()));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!(((factor_0.with(|v| v.as_pointer())) != ({ b_factor_address_2() })));
    assert!(((FnPtr::<fn(i32) -> i32>::new(scaled_1)) == ({ b_scaled_address_3() })));
    assert!((({ scaled_1(2,) }) == 6));
    assert!((({ b_scaled_4(2,) }) == 6));
    return 0;
}
pub fn b_factor_address_2() -> Ptr<i32> {
    return (factor_0.with(|v| v.as_pointer()));
}
pub fn b_scaled_address_3() -> FnPtr<fn(i32) -> i32> {
    return (FnPtr::<fn(i32) -> i32>::new(scaled_1));
}
pub fn b_scaled_4(mut v: i32) -> i32 {
    return ({ scaled_1(v) });
}
pub fn __cpp2rust_init_globals() {
    let _ = factor_0.with(|_| ());
}
