extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_0(mut a: i32, mut b: i32) -> i32 {
    return (a + b);
}
pub fn sub_1(mut a: i32, mut b: i32) -> i32 {
    return (a - b);
}
pub fn mul_2(mut a: i32, mut b: i32) -> i32 {
    return (a * b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fn_: FnPtr<fn(i32, i32) -> i32> = FnPtr::<fn(i32, i32) -> i32>::new(add_0);
    assert!((({ fn_.call(3, 4,) }) == 7));
    fn_ = FnPtr::<fn(i32, i32) -> i32>::new(sub_1);
    assert!((({ fn_.call(10, 3,) }) == 7));
    fn_ = FnPtr::<fn(i32, i32) -> i32>::new(mul_2);
    assert!((({ fn_.call(6, 7,) }) == 42));
    fn_ = FnPtr::<fn(i32, i32) -> i32>::null();
    assert!((fn_).is_null());
    fn_ = FnPtr::<fn(i32, i32) -> i32>::new(add_0);
    assert!(!((fn_).is_null()));
    assert!((({ fn_.call(1, 1,) }) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
