extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cases_and_default_stacked_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 3 => {
                r = 3;
                break 'switch;
            }
            _ => {
                r = 42;
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
    assert!((({ cases_and_default_stacked_0(1,) }) == 42));
    assert!((({ cases_and_default_stacked_0(2,) }) == 42));
    assert!((({ cases_and_default_stacked_0(3,) }) == 3));
    assert!((({ cases_and_default_stacked_0(99,) }) == 42));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
