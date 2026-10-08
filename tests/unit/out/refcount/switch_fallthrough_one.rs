extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_one_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 1 => {
            r += 10;
        }
        __v if __v == 2 => {
            r += 20;
            break;
        }
        _ => {
            r = -1_i32;
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
    assert!((({ fallthrough_one_0(1,) }) == 30));
    assert!((({ fallthrough_one_0(2,) }) == 20));
    assert!((({ fallthrough_one_0(99,) }) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
