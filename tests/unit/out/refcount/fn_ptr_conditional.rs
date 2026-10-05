extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(mut x: i32) -> i32 {
    return (x + 1);
}
pub fn dec_1(mut x: i32) -> i32 {
    return (x - 1);
}
pub fn identity_2(mut x: i32) -> i32 {
    return x;
}
pub fn pick_3(mut mode: i32) -> FnPtr<fn(i32) -> i32> {
    return if (mode > 0) {
        FnPtr::<fn(i32) -> i32>::new(inc_0)
    } else {
        if (mode < 0) {
            FnPtr::<fn(i32) -> i32>::new(dec_1)
        } else {
            FnPtr::<fn(i32) -> i32>::new(identity_2)
        }
    };
}
pub fn apply_4(mut fn_: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    let mut actual: FnPtr<fn(i32) -> i32> = if !(fn_).is_null() {
        (fn_).clone()
    } else {
        FnPtr::<fn(i32) -> i32>::new(identity_2)
    };
    return ({ actual.call(x) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ ({ pick_3(1,) }).call(10,) }) == 11));
    assert!((({ ({ pick_3(-1_i32,) }).call(10,) }) == 9));
    assert!((({ ({ pick_3(0,) }).call(10,) }) == 10));
    assert!((({ apply_4(FnPtr::<fn(i32) -> i32>::new(inc_0), 5,) }) == 6));
    assert!((({ apply_4(FnPtr::<fn(i32) -> i32>::null(), 5,) }) == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
