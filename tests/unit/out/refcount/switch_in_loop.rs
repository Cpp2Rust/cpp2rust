extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_in_loop_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        'switch: {
            match { (i % 3) } {
                __v if __v == 0 => {
                    r += 1;
                    break 'switch;
                }
                __v if __v == 1 => {
                    r += 2;
                    break 'switch;
                }
                _ => {
                    r += 3;
                    break 'switch;
                }
            }
        };
        r += 10;
        i.prefix_inc();
    }
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ switch_in_loop_0(6,) }) == 72));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
