extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn empty_case_with_break_0(mut x: i32) -> i32 {
    let mut r: i32 = 5;
    'switch: {
        match { x } {
            __v if __v == 1 => {
                break 'switch;
            }
            __v if __v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = 9;
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
    assert!((({ empty_case_with_break_0(1,) }) == 5));
    assert!((({ empty_case_with_break_0(2,) }) == 2));
    assert!((({ empty_case_with_break_0(9,) }) == 9));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
