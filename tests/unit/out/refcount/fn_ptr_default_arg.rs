extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(mut x: i32) -> i32 {
    return x;
}
pub fn apply_1(mut x: i32, fn_: Option<FnPtr<fn(i32) -> i32>>) -> i32 {
    let mut fn_: FnPtr<fn(i32) -> i32> = fn_.unwrap_or_else(|| FnPtr::<fn(i32) -> i32>::null());
    if !(fn_).is_null() {
        return ({ fn_.call(x) });
    }
    return x;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ apply_1(5, None,) }) == 5));
    assert!((({ apply_1(5, Some(FnPtr::<fn(i32) -> i32>::null()),) }) == 5));
    assert!((({ apply_1(5, Some(FnPtr::<fn(i32) -> i32>::new(identity_0)),) }) == 5));
    let mut negate: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|x: i32| -> i32 {
        {
            return -x;
        }
    });
    assert!((({ apply_1(5, Some((negate).clone()),) }) == -5_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
