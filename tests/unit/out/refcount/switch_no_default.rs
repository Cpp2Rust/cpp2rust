extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn no_default_0(mut x: i32) -> i32 {
    let mut r: i32 = -1_i32;
    'switch: {
        match { x } {
            __v if __v == 7 => {
                r = 1;
                break 'switch;
            }
            __v if __v == 8 => {
                r = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ no_default_0(7,) }) == 1));
    assert!((({ no_default_0(42,) }) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
