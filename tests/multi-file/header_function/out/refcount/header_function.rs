extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ helper_0(42,) }) == 43) as i32) != 0));
    return 0;
}
pub fn unrelated1_1() -> i32 {
    return 1;
}
pub fn unrelated2_2() -> i32 {
    return 2;
}
pub fn unrelated3_3() -> i32 {
    return 3;
}
pub fn helper_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    ({ unrelated1_1() });
    ({ unrelated2_2() });
    ({ unrelated3_3() });
    return ((*x.borrow()) + 1);
}
