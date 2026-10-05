extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn default_then_case_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 1 => {
                r = 1;
                break 'switch;
            }
            __v if __v == 3 => {
                r = 3;
                break 'switch;
            }
            _ => {
                r = 77;
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
    assert!((({ default_then_case_0(1,) }) == 1));
    assert!((({ default_then_case_0(2,) }) == 77));
    assert!((({ default_then_case_0(3,) }) == 3));
    assert!((({ default_then_case_0(99,) }) == 77));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
