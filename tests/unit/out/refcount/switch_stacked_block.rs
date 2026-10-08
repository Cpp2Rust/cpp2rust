extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn stacked_block_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            __v if __v == 1 || __v == 2 || __v == 3 => {
                let mut y: i32 = (x * 2);
                r = (y + 1);
                break 'switch;
            }
            _ => {
                r = 0;
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
    assert!((({ stacked_block_0(2,) }) == 5));
    assert!((({ stacked_block_0(9,) }) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
