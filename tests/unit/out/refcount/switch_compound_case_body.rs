extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn compound_case_body_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 1 => {
                let mut y: i32 = 10;
                let mut z: i32 = 20;
                r = (y + z);
                break 'switch;
            }
            __v if __v == 2 => {
                let mut y: i32 = 100;
                r = (y - 1);
                break 'switch;
            }
            _ => {
                r = -1_i32;
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
    assert!((({ compound_case_body_0(1,) }) == 30));
    assert!((({ compound_case_body_0(2,) }) == 99));
    assert!((({ compound_case_body_0(9,) }) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
