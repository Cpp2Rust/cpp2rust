extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn case_then_default_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 2 => {
                r = 20;
                break 'switch;
            }
            _ => {
                r = 10;
                break 'switch;
            }
        }
    };
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ case_then_default_0(1,) }) == 10));
    assert!((({ case_then_default_0(2,) }) == 20));
    assert!((({ case_then_default_0(99,) }) == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
