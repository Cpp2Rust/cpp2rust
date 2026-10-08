extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_switch_for_break_0(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        'switch: {
            match { i } {
                __v if __v == 1 => {
                    let mut j: i32 = 0;
                    'loop_: while (j < 10) {
                        if (j == 2) {
                            break;
                        }
                        r += 1;
                        j.prefix_inc();
                    }
                    r += 100;
                    break 'switch;
                }
                _ => {
                    r += 10;
                    break 'switch;
                }
            }
        };
        i.prefix_inc();
    }
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ for_switch_for_break_0(3,) }) == 122));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
