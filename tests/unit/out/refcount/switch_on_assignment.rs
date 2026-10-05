extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_on_assignment_0(mut x: i32) -> i32 {
    let mut y: i32 = 0;
    let mut r: i32 = 0;
    'switch: {
        match {
            {
                y = (x + 1);
                y
            }
        } {
            __v if __v == 1 => {
                r = 10;
                break 'switch;
            }
            __v if __v == 2 => {
                r = 20;
                break 'switch;
            }
            _ => {
                r = y;
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
    assert!((({ switch_on_assignment_0(0,) }) == 10));
    assert!((({ switch_on_assignment_0(1,) }) == 20));
    assert!((({ switch_on_assignment_0(9,) }) == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
