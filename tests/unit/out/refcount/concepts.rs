extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_small_0() -> bool {
    return true;
}
pub fn is_small_1() -> bool {
    return false;
}
pub fn pick_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    if (true) && (true) {
        return 1;
    }
    return 2;
}
pub fn pick_3(x: i64) -> i32 {
    let x: Value<i64> = Rc::new(RefCell::new(x));
    if (true) && (false) {
        return 1;
    }
    return 2;
}
pub fn pick_4(x: f32) -> i32 {
    let x: Value<f32> = Rc::new(RefCell::new(x));
    if (false) && (true) {
        return 1;
    }
    return 2;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ is_small_0() }));
    assert!(!({ is_small_1() }));
    assert!((({ pick_2(1,) }) == 1));
    assert!((({ pick_3(1_i64,) }) == 2));
    assert!((({ pick_4(1.0E+0,) }) == 2));
    return 0;
}
