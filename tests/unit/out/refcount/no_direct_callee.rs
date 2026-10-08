extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0() -> bool {
    return false;
}
pub fn test_1(mut fn_: FnPtr<fn() -> bool>) -> i32 {
    if !({ fn_.call() }) {
        return 1;
    }
    return 0;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ test_1(FnPtr::<fn() -> bool>::new(test1_0),) }) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
