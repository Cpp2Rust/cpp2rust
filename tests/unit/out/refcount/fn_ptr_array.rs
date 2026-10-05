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
    let mut ops: [FnPtr<fn(i32, i32) -> i32>; 3] = [
        FnPtr::<fn(i32, i32) -> i32>::new(add_0),
        FnPtr::<fn(i32, i32) -> i32>::new(sub_1),
        FnPtr::<fn(i32, i32) -> i32>::new(mul_2),
    ];
    assert!((({ ops[(0) as usize].call(2, 3,) }) == 5));
    assert!((({ ops[(1) as usize].call(7, 4,) }) == 3));
    assert!((({ ops[(2) as usize].call(6, 5,) }) == 30));
    assert!(!((ops[(0) as usize]).is_null()));
    assert!((ops[(0) as usize] == FnPtr::<fn(i32, i32) -> i32>::new(add_0)));
    assert!((ops[(0) as usize] != FnPtr::<fn(i32, i32) -> i32>::new(sub_1)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
