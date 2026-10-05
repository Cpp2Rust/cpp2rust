extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_default_0(mut x: i32, mut flag: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 7 => {
            if (flag != 0) {
                r = 100;
                break;
            };
        }
        _ => {
            r = 42;
            break;
        }
    });
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ fallthrough_default_0(7, 0,) }) == 42));
    assert!((({ fallthrough_default_0(7, 1,) }) == 100));
    assert!((({ fallthrough_default_0(99, 0,) }) == 42));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
