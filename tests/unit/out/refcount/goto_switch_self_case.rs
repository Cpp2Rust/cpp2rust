extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sm_0(mut n: i32) -> i32 {
    let mut steps: i32 = 0;
    switch!(match n {
        __v if __v == 0 => 'target: {
            steps += 1;
            break;
        }
        __v if __v == 1 => {
            steps += 10;
            goto!('target);
        }
        _ => {
            steps = -1_i32;
            break;
        }
    });
    return steps;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ sm_0(0,) }) == 1) as i32) != 0));
    assert!((((({ sm_0(1,) }) == 11) as i32) != 0));
    assert!((((({ sm_0(7,) }) == -1_i32) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
