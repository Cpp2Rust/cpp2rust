extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_chain_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 1 => {
            r += 1;
        }
        __v if __v == 2 => {
            r += 2;
        }
        __v if __v == 3 => {
            r += 4;
        }
        __v if __v == 4 => {
            r += 8;
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
    assert!((({ fallthrough_chain_0(1,) }) == 15));
    assert!((({ fallthrough_chain_0(2,) }) == 14));
    assert!((({ fallthrough_chain_0(3,) }) == 12));
    assert!((({ fallthrough_chain_0(4,) }) == 8));
    assert!((({ fallthrough_chain_0(99,) }) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
